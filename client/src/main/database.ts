import { ipcMain } from 'electron';

export default function useDatabase() {
  let counter = 0;

  ipcMain.handle('connect-to-database', async (event, arg) => {
    console.log('connect-to-database', arg);

    counter += 1;
    return counter;
  });
}
