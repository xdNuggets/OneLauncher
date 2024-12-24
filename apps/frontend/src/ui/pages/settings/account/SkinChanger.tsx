import type { MinecraftSkin } from "@onelauncher/client/bindings";
import { CheckIcon, PlusIcon, Trash01Icon } from "@untitled-theme/icons-solid";
import { createContext, createSignal, For, useContext, type Context, type ParentProps, type Resource } from "solid-js";
import { bridge } from "~imports";
import Button from "~ui/components/base/Button";
import TextField from "~ui/components/base/TextField";
import Tooltip from "~ui/components/base/Tooltip";
import useAccountController from "~ui/components/overlay/account/AddAccountModal";
import Modal, { createModal, type ModalProps } from "~ui/components/overlay/Modal";
import useCommand, { tryResult } from '~ui/hooks/useCommand';
import useNotifications from "~ui/hooks/useNotifications";


const PlayerModel = ({src, limitSize} : {src: string, limitSize?: boolean}) => {
	return <img src={"https://vzge.me/full/384/" + src} alt="model" class={limitSize ? "h-60 w-37" : ""} />
}

export default function SkinChangerPage() {
	const skinController = useSkinController();
	const skins = skinController.skins;


	const CurrentSkinDisplay = () => {
		const accountController = useAccountController();
		const currentSkin = skinController.currentSkin;
		return (
			<div class="flex ml-[10px] flex-col items-center">
				<p class="text-2lg">Current skin</p>
				<PlayerModel src={currentSkin()!!.src ?? accountController.defaultAccount()?.skin.src}/>
				<div>
					<Tooltip title="Add Skin" text="Add Skin" position="bottom">
						<FileUploadButton/>
					</Tooltip>
				</div>
			</div>
		)
	}

	return (
		<div class={`flex items-center h-full justify-around`}>
			{/* Current Skin / Add new one */}
			<CurrentSkinDisplay/>
			{/* Skin Library */}
				<div class="grid gap-x-20 gap-y-5 md:grid-cols-6 sm:grid-cols-3">
					<For each={skins() ?? []} fallback={<div>No skins found.</div>}>
						{skin => (
							<SkinDisplayComponent skin={skin}/>
						)}
					</For>
				</div>
		</div>
	)
}



interface SkinDisplayProps {
	skin: MinecraftSkin;
}

function SkinDisplayComponent(props: SkinDisplayProps) {

	const skinController = useSkinController();
	const notificationController = useNotifications();

	const SelectSkinButton = () => {
		return (
			<Tooltip title="Set Skin" text={props.skin.current ? "This skin is active!" : "Set skin"} position="bottom">
				<Button buttonStyle="iconPrimary" disabled={props.skin.current} onClick={() => {
					notificationController.set("skin_set_current", {
						title: "Current skin set",
						message: `Current skin set to ${props.skin.name}`
					})

					useCommand(() => bridge.commands.setSkin(props.skin))
					skinController.refetch();
					location.reload();
					}}>
						<CheckIcon/></Button>
			</Tooltip>
		)
	}

	const DeleteSkinButton = () => {
		const deleteModal = createModal((p: ModalProps) => (
			<Modal.Delete {...p} timeLeft={0} title="Delete Skin?" onDelete={() => {
				skinController.removeSkin(props.skin.id)
				notificationController.set("skin_delete", {
					title: "Deleted skin",
					message: `Removed skin ${props.skin.name}`
				})

				skinController.refetch();
			}} >
				<div>
					<p>Are you sure you want to delete this skin?</p>

				</div>
			</Modal.Delete>
		));


		return (
			<Tooltip title="Delete Skin" text={props.skin.current ? "This skin is active!" : "Delete Skin"} position="bottom">
				<Button buttonStyle="iconDanger" disabled={props.skin.current} onClick={() => deleteModal.show()}>
					<Trash01Icon/>
				</Button>
			</Tooltip>
		)
	}

	return (
		<div>
			{/* Skin Display */}
			<div class="flex flex-col items-center">
				<p class="text-lg">{props.skin.name}</p>
				<PlayerModel limitSize src={props.skin.src}/>
			</div>
			{/* Buttons */}
			<div class="gap-4 flex justify-center mt-2">
				<SelectSkinButton />
				<DeleteSkinButton />
			</div>
		</div>
	)
}

const FileUploadButton = () => {
	const skinController = useSkinController();
	const [inputRef, setInputRef] = createSignal<HTMLInputElement | null>(null);
	const [skinName, setSkinName] = createSignal<string>("");
	const [encodedFile, setEncodedFile] = createSignal("");

	const inputNameModal = createModal((props: ModalProps) => {
		const [modalProps, _p] = Modal.SplitProps(props);

		return (
			<Modal.Simple {...modalProps} title="Name your skin">
				<TextField
					placeholder="Skin Name"
					onValidSubmit={(input) => {
						setSkinName(input);

						const uuid = crypto.randomUUID();
						const skin: MinecraftSkin = {
							id: uuid,
							name: skinName(),
							src: encodedFile(),
							current: false,
						};

						useCommand(() => bridge.commands.addSkin(skin));
						skinController.refetch();
						modalProps.hide();
					}}
				/>
			</Modal.Simple>
		);
	});

	function onFileChange(e: Event) {
		const target = e.target as HTMLInputElement;
		const file = target.files?.[0];
		// Encode here
		if (file) {
			const reader = new FileReader();
			reader.onload = () => {
				const encodedString = (reader.result as string).split(",")[1];
				if(!encodedString) return;
				setEncodedFile(encodedString);
				setSkinName("");
				inputNameModal.show();

				target.value = "";

			};

			reader.onerror = (error) => {
				console.error("Error encoding file:", error);
			};

			reader.readAsDataURL(file);
		} else {
			console.error("No file selected");
		}


	}

	return (
		<>
			<Button buttonStyle="icon" onClick={() => inputRef()?.click()}><PlusIcon/></Button>
			<input type="file" onChange={onFileChange} ref={setInputRef} style={{display: "none"}} />
		</>
	)
}



interface SkinControllerContextFunc {
	currentSkin: Resource<MinecraftSkin | null>;
	skins: Resource<MinecraftSkin[]>;
	addSkin: (skin: MinecraftSkin) => Promise<void>;
	removeSkin: (uuid: string) => Promise<void>;
	setSkin: (skin: MinecraftSkin) => Promise<void>;
	getSkin: (uuid: string) => Promise<MinecraftSkin>;
	refetch: () => void;
}

const SkinControllerContext = createContext<SkinControllerContextFunc>() as Context<SkinControllerContextFunc>;

export function SkinControllerProvider(props: ParentProps) {

	async function addSkin(skin: MinecraftSkin) {
		await tryResult(() => bridge.commands.addSkin(skin));
	}

	async function removeSkin(uuid: string) {
		await tryResult(() => bridge.commands.removeSkin(uuid));
	}

	async function setSkin(skin: MinecraftSkin) {
		await tryResult(() => bridge.commands.setSkin(skin));
		refetch();
	}

	async function getSkin(uuid: string) {
		return await tryResult(() => bridge.commands.getSkin(uuid));
	}


	const [skins, { refetch: refetchSkins }] = useCommand(() => bridge.commands.getSkins());
	const [currentSkin, { refetch: refetchCurrentSkin }] = useCommand(() => bridge.commands.getCurrentSkin())

	function refetch() {
		refetchSkins();
		refetchCurrentSkin();
	}

	const func: SkinControllerContextFunc = {
		currentSkin,
		skins,
		addSkin,
		removeSkin,
		setSkin,
		getSkin,
		refetch,
	};

	return (
		<SkinControllerContext.Provider value={func}>
			{props.children}
		</SkinControllerContext.Provider>
	)
}


export function useSkinController() {
	return useContext(SkinControllerContext);
}
