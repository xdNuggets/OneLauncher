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


const PlayerModel = ({src, limitSize} : {src: string, limitSize?: boolean}) => {
	return <img src={"https://vzge.me/full/384/" + src} alt="model" class={limitSize ? "h-60 w-37" : ""} />
}

export default function SkinChangerPage() {
	const skinController = useSkinController();
	const accountController = useAccountController();
	const skins = skinController.skins;
	const currentSkin = skinController.currentSkin == null ? accountController.defaultAccount()?.skin : skinController.currentSkin;

	return (
		<div class={`flex items-center h-full ${skins.length != 0 ? "justify-around" : ""}`}>
			{/* Current Skin / Add new one */}
			<div class="flex ml-[10px] flex-col items-center">
				<p class="text-2lg">Current skin</p>
				<PlayerModel src={currentSkin!!.src}/>
				<Tooltip title="Add Skin" text="Add Skin" position="bottom">
					<FileUploadButton refetch={skinController.refetch}/>
				</Tooltip>
			</div>

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

	const SelectSkinButton = () => {
		return (
			<Button buttonStyle="iconPrimary" onClick={() => {useCommand(() => bridge.commands.setSkin(props.skin))}}><CheckIcon/></Button>
		)
	}

	const DeleteSkinButton = () => {
		return (
			<Button buttonStyle="iconDanger" onClick={() => {useCommand(() => bridge.commands.removeSkin(props.skin.id))}} class=""><Trash01Icon/></Button>
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

const FileUploadButton = ({refetch} : {refetch: () => void}) => {
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
						refetch();
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
	currentSkin: MinecraftSkin | undefined;
	skins: Resource<MinecraftSkin[]>;
	addSkin: (skin: MinecraftSkin) => Promise<void>;
	removeSkin: (uuid: string) => Promise<void>;
	setSkin: (skin: MinecraftSkin) => Promise<void>;
	getSkin: (uuid: string) => Promise<MinecraftSkin>;
	refetch: () => void;
}

const SkinControllerContext = createContext<SkinControllerContextFunc>() as Context<SkinControllerContextFunc>;

export function SkinControllerProvider(props: ParentProps) {
	const accountController = useAccountController();

	async function addSkin(skin: MinecraftSkin) {
		await tryResult(() => bridge.commands.addSkin(skin));
	}

	async function removeSkin(uuid: string) {
		await tryResult(() => bridge.commands.removeSkin(uuid));
	}

	async function setSkin(skin: MinecraftSkin) {
		await tryResult(() => bridge.commands.setSkin(skin));
	}

	async function getSkin(uuid: string) {
		return await tryResult(() => bridge.commands.getSkin(uuid));
	}


	const [skins, {refetch: refetchSkins} ] = useCommand(() => bridge.commands.getSkins());

	function refetch() {
		refetchSkins();
	}

	const func: SkinControllerContextFunc = {
		currentSkin: accountController.defaultAccount()?.skin,
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
