import "./app.css";
import {invoke} from "@tauri-apps/api/tauri";
import {useEffect, useState} from "react";

function App() {
	const [wallpapers, setWallpapers] = useState<string[]>([])


	useEffect(() => {
		invoke<string[]>("get_wallpapers").then(r => setWallpapers(r))
	}, [])



	return (
		<>{
			wallpapers.map(v => <p>{v}</p>)
		}</>
	);
}

export default App;
