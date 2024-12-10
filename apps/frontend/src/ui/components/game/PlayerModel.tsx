import { createEffect, createSignal, type JSX, splitProps } from "solid-js";


type PlayerModelProps = JSX.IntrinsicElements['img'] & {
	uuid?: string | null | undefined;
	skin?: File;
	onError?: () => void;
};

async function getSkin(uuid: string) : Promise<string> {
	const url = `https://api.mineatar.io/body/full/${uuid}?scale=8`;
	const response = await fetch(url);
	const blob = await response.blob();
	return URL.createObjectURL(blob);

}


export default function PlayerModel(props: PlayerModelProps) {
	const [split, rest] = splitProps(props, ['uuid', 'skin', 'class']);
	const [skin, setSkin] = createSignal<string>('');

	createEffect(async () => {
		if (split.skin) {
			setSkin('');
		} else if (split.uuid) {
			const src = await getSkin(split.uuid);
			setSkin(src);
		} else {
			props.onError?.();
		}
	});

	return (
		<img
			class={`image-render-pixel ${split.class}`}
			src={skin()}
			{...rest}
		/>
	);
}
