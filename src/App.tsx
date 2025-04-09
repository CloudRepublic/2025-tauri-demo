import {useState} from "react";
import "./App.css";
import { commands } from "./bindings";

function App() {
    const [qrImage, setQrImage] = useState("");
    const [name, setName] = useState("");

    async function GenerateQrCode() {
        try {
            let qr = await commands.qr(name);
            setQrImage(qr.base64_image);
        } catch (error) {
            console.error('Error calling GenerateQrCode:', error);
        }
    }

    return (
        <main className="container">
            <h1>Welcome to Tauri + React</h1>
            <input
                id="greet-input"
                onChange={(e) => {
                    setName(e.currentTarget.value);
                    setTimeout(() => {
                        GenerateQrCode();
                    }, 0);
                }} placeholder="Enter a name..."
            />
            <br/>
            {qrImage !== "" && (
                <img style={{height: "50vh"}} src={qrImage} alt="QR code"/>
            )}
        </main>
    );
}

export default App;
