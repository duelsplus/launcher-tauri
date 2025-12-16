import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Form, Stack, Button, TextInput } from "@carbon/react";
import "./styles/carbon.scss";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main>
      <Form
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <Stack gap={4} orientation="horizontal">
          <TextInput
            id="greet-input"
            labelText="Name"
            size="md"
            type="text"
            inline
            onChange={(e) => setName(e.currentTarget.value)}
          />
          <Button type="submit">Greet</Button>
        </Stack>
      </Form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
