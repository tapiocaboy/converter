import { ws_ping } from './pkg';

async function run() {
    await import('./pkg/converter.js');
    
    const wsEndpoint = "ws://localhost:8080";
    const defaultMessage = "Type a message";

    const container = document.createElement('div');
    container.innerHTML = `
        <div>
            <div>Address: <input type="text" id="wsEndpoint" value="${wsEndpoint}"></div>
            <div>Message: <input type="text" id="message" value="${defaultMessage}"></div>
            <div><button id="sendBtn">Send</button></div>
            <div id="response"></div>
        </div>
    `;
    document.body.appendChild(container);

    document.getElementById('sendBtn').addEventListener('click', async () => {
        const endpoint = document.getElementById('wsEndpoint').value;
        const msg = document.getElementById('message').value;
        const responseDiv = document.getElementById('response');

        try {
            responseDiv.textContent = 'SENDING';
            const response = await ws_ping(endpoint, msg);
            responseDiv.textContent = `RESPONSE: ${response}`;
        } catch (error) {
            responseDiv.textContent = `ERROR: ${error.message || error}`;
            console.error('ERROR:', error);
        }
    });
}

run().catch(console.error);
