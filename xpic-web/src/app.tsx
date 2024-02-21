import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import "./app.css";

function App() {
	const [wallpapers, setWallpapers] = useState<string[]>([]);

	useEffect(() => {
		invoke<string[]>("get_wallpapers").then((wallpapers) =>
			setWallpapers(wallpapers),
		);
	}, []);

	return (
		<>
			{wallpapers.map((v) => (
				<img src={convertFileSrc(v)} alt={v} />
			))}
		</>
	);
}

export default App;
