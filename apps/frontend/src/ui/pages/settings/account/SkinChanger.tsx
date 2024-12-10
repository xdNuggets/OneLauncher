import { PlusIcon } from "@untitled-theme/icons-solid";
import Button from "~ui/components/base/Button";
import Tooltip from "~ui/components/base/Tooltip";
import PlayerModel from "~ui/components/game/PlayerModel";
import useAccountController from "~ui/components/overlay/account/AddAccountModal";


export default function SkinChangerPage() {
	const accountController = useAccountController();

	return (
		<div class="flex items-center h-full">
			{/* Current Skin / Add new one */}
			<div class="flex ml-[10px] flex-col items-center">
				<PlayerModel uuid={accountController.defaultAccount()?.id} class="pb-3" />
				<Tooltip title="Add Skin" text="Add Skin" position="bottom">
					<Button buttonStyle="icon">
						<PlusIcon />
					</Button>
				</Tooltip>
			</div>

			{/* Skin Library */}
			<div>

			</div>
		</div>
	)
}


interface SkinDisplayProps {
	skin: File;
}

function SkinDisplayComponent(props: SkinDisplayProps) {
	return (
		<div class="flex flex-col items-center">
			<PlayerModel skin={props.skin} onError={() => console.log('Error loading skin')} />
			<Button buttonStyle="icon">

			</Button>
		</div>
	)
}
