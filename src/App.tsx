import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
	const [list, setList] = useState([])

	useEffect(() => {
		async function name() {
			const e = await invoke("refresh_sessions");
			console.log(e);
		}
		name()
	}, [])
	
	return (
		<main className="container">
			<h1>Hi!</h1>
		</main>
	);
}

export default App;
