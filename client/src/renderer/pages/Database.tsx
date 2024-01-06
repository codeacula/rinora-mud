import React from 'react';

export default function Database() {
  async function connectToDatabase(event: React.FormEvent) {
    event.preventDefault();
    const formData = new FormData(event.target as HTMLFormElement);
    const request = {
      host: formData.get('host'),
      port: formData.get('port') || 5432,
      username: formData.get('username'),
      password: formData.get('password'),
    };
    console.log('Sending message to main process', request);
    const result = await window.electron.ipcRenderer.invoke(
      'connect-to-database',
      request,
    );
    console.log('Got result from main process', result);
  }
  return (
    <div>
      <form onSubmit={connectToDatabase}>
        <h2>Connect To The Database</h2>
        <div>
          <label htmlFor="host" defaultValue="localhost">
            Host <input name="host" />
          </label>
          <label htmlFor="port">
            Port <input name="port" defaultValue={5432} type="number" />
          </label>
          <label htmlFor="username" defaultValue="dev">
            Username <input name="username" />
          </label>
          <label htmlFor="password">
            Password <input name="password" type="password" />
          </label>
          <button type="submit">Connect</button>
        </div>
      </form>
    </div>
  );
}
