import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
	const [list, setList] = useState<{ name: string, created: string }[]>([])
	const [pattern, setPattern] = useState<string>("kitty <>");
	const [newName, setNewName] = useState<string>("");

	async function update() {
		await invoke("refresh_sessions");
		const e: { name: string, created: string }[] = JSON.parse(await invoke("get_sessions"));
		setList(e);
	}

	function formCommand(call: String) {
		let actual = call.match(/('[a-zA-Z_\-0-9 ]+')|[a-zA-Z_\-0-9]+/g) ?? [];
		let command: { name: string, args: string[] } = {
			name: actual[0] ?? "shouldnt occur",
			args: [

			],
		}
		actual.splice(0, 1);
		actual.forEach(v => command.args.push(v));

		return command;
	}

	async function runAttachCommand(name: string) {
		const command = formCommand(pattern.replace("<>", `tmux attach -t ${name}`))
		console.log(await invoke("run_command", { command: command }));
		update();
	}
	async function runKillCommand(name: string) {
		const command = formCommand(`tmux kill-session -t ${name}`)
		console.log(await invoke("run_command", { command: command }));
		update();
	}
	async function runAddCommand() {
		if (newName == "") return;

		console.log(newName);
		
		const command = formCommand(`tmux new-session -d -t ${newName}`)
		console.log(await invoke("run_command", { command: command }));
		update();
	}

	// setInterval(() => {
	// 	update()
	// }, 1500)

	update()

	return (
		<main className="container">
			<div className="list">
				{list?.map(v => <div className="item" key={v.name}>
					<div>{v.name}</div>
					<div>{v.created}</div>
					<div className="buttons">
						<button onClick={() => runAttachCommand(v.name)}>Attach</button>
						<button onClick={() => runKillCommand(v.name)}>Kill</button>
					</div>
				</div>)}
			</div>
			<div className="pin">
				<div className="settings">
					<label>terminal pattern: </label>
					<input type="text" value={pattern} onChange={(e) => setPattern(e.target.value)} placeholder="konsole -e '<>'" />
				</div>
				<div className="add">
					<button onClick={() => runAddCommand()}>Add</button>
					<input type="text" placeholder="Name of new session" value={newName} onChange={(e) => setNewName(e.target.value)} />
				</div>
			</div>
		</main>
	);
}

export default App;
