import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import "lightgallery/css/lightgallery-bundle.css";
import lgThumbnail from "lightgallery/plugins/thumbnail";
import lgZoom from "lightgallery/plugins/zoom";
import LightGallery from "lightgallery/react";
import "overlayscrollbars/overlayscrollbars.css";
import { useEffect, useRef, useState } from "react";
import "./app.css";
import {
	OverlayScrollbarsComponent,
	OverlayScrollbarsComponentRef,
} from "overlayscrollbars-react";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";

function App() {
	const [wallpapers, setWallpapers] = useState<string[]>([]);

	useEffect(() => {
		invoke<string[]>("get_wallpapers").then((wallpapers) =>
			setWallpapers(wallpapers),
		);

		invoke<string[]>("update_wallpapers").then((wallpapers) =>
			setWallpapers(wallpapers),
		);

		if (import.meta.env.MODE !== "development") {
			document.addEventListener("contextmenu", (event) =>
				event.preventDefault(),
			);
		}
	}, []);

	const getName = (path: string): string => {
		const match = path.trim().match(/[^/\\]+$/);
		return match ? match[0] : "";
	};

	const [width, height] = [240, 135];
	const cols = 4;
	const rows = 4;
	const gap = 32;
	const px = 64;
	const py = 32;

	const layout = () => {
		const gallery = document.querySelector<HTMLDivElement>(".custom-gallery");
		// cols * width + (cols - 1) * gap + px * 2
		const cols = Math.floor((window.innerWidth - px * 2 + gap) / (width + gap));
		if (gallery) {
			gallery.style.gridTemplateColumns = `repeat(${cols}, minmax(0, 1fr))`;
		}
	};

	useEffect(() => {
		layout();
		window.addEventListener("resize", layout);

		const size = new LogicalSize(
			cols * width + (cols - 1) * gap + px * 2,
			rows * height + (rows - 1) * gap + py * 2,
		);

		appWindow.setSize(size).then(() => {
			void appWindow.center();
		});

		const minSize = new LogicalSize(width + px * 2, height + py * 2);
		void appWindow.setMinSize(minSize);

		console.log(size.width, size.height);

		return () => {
			window.removeEventListener("resize", layout);
		};
	}, []);

	const osRef = useRef<OverlayScrollbarsComponentRef>(null);
	// useEffect(() => {
	//     if (!osRef.current) {
	//         return
	//     }
	//
	//     const osIns = osRef.current.osInstance();
	//     if (!osIns) {
	//         return;
	//     }
	// }, [osRef]);

	return (
		<>
			<OverlayScrollbarsComponent ref={osRef} options={{}} defer>
				<div className={"w-screen h-screen grid items-center justify-center"}>
					<LightGallery
						elementClassNames={`custom-gallery grid gap-8 items-center justify-center py-8 px-16`}
						plugins={[lgZoom, lgThumbnail]}
						speed={500}
						thumbnail={true}
						thumbWidth={width}
						thumbHeight={`${height}`}
					>
						{wallpapers.map((v) => (
							<a href={convertFileSrc(v)} draggable={false}>
								<img
									src={convertFileSrc(v)}
									alt={getName(v)}
									width={width}
									height={height}
									className={"rounded hover:brightness-125 hover:drop-shadow"}
								/>
							</a>
						))}
					</LightGallery>
				</div>
			</OverlayScrollbarsComponent>
		</>
	);
}

export default App;
