import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
	const [list, setList] = useState<{ name: string, created: string }[]>([])

	useEffect(() => {
		async function name() {
			await invoke("refresh_sessions");
			const e: { name: string, created: string }[] = JSON.parse(await invoke("get_sessions"));
			setList(e);
		}
		name()
	}, [])

	console.log(typeof(list));
	
	return (
		<main className="container">
			<div className="list">
				{list?.map(v => <div className="item">
					<div>{v.name}</div>
					<div>{v.created}</div>
				</div>)}
			</div>
		</main>
	);
}

export default App;
