import type { MinecraftSkin } from "@onelauncher/client/bindings";
import { CheckIcon, PlusIcon, Trash01Icon } from "@untitled-theme/icons-solid";
import { createContext, createEffect, createSignal, useContext, type Context, type ParentProps, type ResourceReturn } from "solid-js";
import { bridge } from "~imports";
import Button from "~ui/components/base/Button";
import Tooltip from "~ui/components/base/Tooltip";
import useAccountController from "~ui/components/overlay/account/AddAccountModal";
import useCommand, { tryResult } from '~ui/hooks/useCommand';


const PlayerModel = ({src, limitSize} : {src: string, limitSize?: boolean}) => {
	return <img src={"https://vzge.me/full/384/" + src} alt="model" class={limitSize ? "h-60 w-37" : ""} />
}

const sampleSkins: MinecraftSkin[] = [
	{id: "1", name: "JoshDoesOF", src: "JoshDoesOF"},
	{id: "2", name: "JoshDoesOF", src: "Notch"},
	{id: "3", name: "JoshDoesOF", src: "JustThiemo"},
	{id: "4", name: "JoshDoesOF", src: "hypixel"},
	{id: "5", name: "JoshDoesOF", src: "DongerOfDongs"},
	{id: "6", name: "JoshDoesOF", src: "JoshDoesOF"},
]


export default function SkinChangerPage() {
	// TODO: Implement adding skin to the account by downloading it from some place idfk
	const accountController = useAccountController();
	const skinController = useSkinController();
	const [skins, setSkins] = createSignal<MinecraftSkin[]>([]);

	createEffect(async () => {
		const skins = await skinController.getSkins();
		console.log("current account: " +accountController.defaultAccount.latest?.skin.name);
		console.log("skin" + skinController.currentSkin?.name);
		if(skins) {
			setSkins(skins);
		} else {
			setSkins([])
		}
	})


	return (
		<div class="flex items-center h-full justify-between">
			{/* Current Skin / Add new one */}
			<div class="flex ml-[10px] flex-col items-center">
				<p class="text-2lg">Current skin</p>
				<PlayerModel src={skinController.currentSkin!!.src}/>
				<Tooltip title="Add Skin" text="Add Skin" position="bottom">
					<FileUploadButton/>
				</Tooltip>
			</div>

			{/* Skin Library */}
			<div class="grid gap-x-20 gap-y-5 md:grid-cols-6 sm:grid-cols-3">

				{sampleSkins.length != 0 ? sampleSkins.map(skin => <SkinDisplayComponent skin={skin} />) : <div>No skins found</div>}
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

const FileUploadButton = () => {
	const inputRef = createSignal<HTMLInputElement | null>(null);

	function onFileChange(e: Event) {
		const target = e.target as HTMLInputElement;
		const file = target.files?.[0];
		// Encode here

		if (file) {
			const reader = new FileReader();

			reader.onload = () => {
				const encodedString = (reader.result as string).split(",")[1];
				if(!encodedString) { return; }
				const uuid = crypto.randomUUID();
				const skin: MinecraftSkin = {
					id: uuid,
					name: uuid + file.name,
					src: encodedString!!,
				};

				useCommand(() => bridge.commands.addSkin(skin));
			};

			reader.onerror = (error) => {
				console.error("Error encoding file:", error);
			};

			reader.readAsDataURL(file);
		} else {
			console.error("No file selected");
		}


	}

	function handleButtonClick() {
		if (inputRef[0]()) {
			inputRef[0]()!!.click();
		}
	}

	return (
		<>
			<Button buttonStyle="icon" onClick={handleButtonClick}><PlusIcon/></Button>
			<input type="file" onChange={onFileChange} ref={inputRef[0]} style={{display: "none"}} onchange={onFileChange} />
		</>
	)
}



interface SkinControllerContextFunc {
	currentSkin: MinecraftSkin | undefined;
	skins: ResourceReturn<MinecraftSkin[]>;
	addSkin: (skin: MinecraftSkin) => Promise<void>;
	removeSkin: (uuid: string) => Promise<void>;
	setSkin: (skin: MinecraftSkin) => Promise<void>;
	getSkin: (uuid: string) => Promise<MinecraftSkin>;
	getSkins: () => Promise<MinecraftSkin[]>;
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

	async function getSkins() {
		return await tryResult(() => bridge.commands.getSkins());
	}

	const func: SkinControllerContextFunc = {
		currentSkin: accountController.defaultAccount.latest?.skin,
		skins: useCommand(() => bridge.commands.getSkins()),
		addSkin,
		removeSkin,
		setSkin,
		getSkin,
		getSkins,
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
