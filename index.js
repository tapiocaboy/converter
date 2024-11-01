import { ws_ping } from './pkg';

async function run() {
    await import('./pkg/converter.js');
    
    const wsEndpoint = "ws://localhost:8080";
    const defaultMessage = "Hello WebSocket!";

    const container = document.createElement('div');
    container.innerHTML = `
        <div>
            <input type="text" id="wsEndpoint" value="${wsEndpoint}" placeholder="WebSocket URL">
            <input type="text" id="message" value="${defaultMessage}" placeholder="Message">
            <button id="sendBtn">Send</button>
            <div id="response"></div>
        </div>
    `;
    document.body.appendChild(container);

    document.getElementById('sendBtn').addEventListener('click', async () => {
        const endpoint = document.getElementById('wsEndpoint').value;
        const msg = document.getElementById('message').value;
        const responseDiv = document.getElementById('response');

        try {
            responseDiv.textContent = 'Sending...';
            const response = await ws_ping(endpoint, msg);
            responseDiv.textContent = `Response: ${response}`;
        } catch (error) {
            responseDiv.textContent = `Error: ${error.message || error}`;
            console.error('WebSocket error:', error);
        }
    });
}

run().catch(console.error);
