const channel = document.querySelector('.channel');

function updateChannel(messages) {
    while (channel.firstChild) {
        channel.removeChild(channel.lastChild);
    }

    for (const message of messages) {
        const { author, timestamp, content, is_edited } = message;
        const messageWrapper = document.createElement('div');
        messageWrapper.className = 'message-wrapper';

        const avatar = document.createElement('img');
        avatar.src = author.avatar_url;
        avatar.className = 'avatar';
        messageWrapper.appendChild(avatar);

        const messageElement = document.createElement('div');
        messageElement.className = 'message';
        messageWrapper.appendChild(messageElement);

        const header = document.createElement('h1');
        header.className = 'header';
        messageElement.appendChild(header);

        const username = document.createElement('span');
        username.className = 'username';
        username.style.color = '#' + author.color.toString(16).padStart(6, '0');
        username.textContent = author.name;
        header.appendChild(username);

        const timestampElement = document.createElement('span');
        timestampElement.className = 'timestamp';
        timestampElement.textContent = String(timestamp);
        header.appendChild(timestampElement);

        const contentElement = document.createElement('p');
        contentElement.className = 'message-content';
        contentElement.textContent = content;
        messageElement.appendChild(contentElement);

        channel.appendChild(messageWrapper);
    }
}
