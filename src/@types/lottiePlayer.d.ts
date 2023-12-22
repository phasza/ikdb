import type { SvelteComponentTyped } from 'svelte';

declare module '@lottiefiles/svelte-lottie-player' {
	interface LottiePlayerProps {
		autoplay: boolean;
		background?: string;
		controls?: boolean;
		controlsLayout?: string;
		count?: number;
		defaultFrame?: 0;
		direction?: 1 | -1;
		height?: number;
		hover?: boolean;
		loop?: boolean;
		mode?: string; //Should be PlayMode.Bounce or Normal
		onToggleZoom?: () => void;
		renderer?: 'svg' | 'canvas';
	}

	export class LottiePlayer extends SvelteComponentTyped<LottiePlayerProps> {}
}
