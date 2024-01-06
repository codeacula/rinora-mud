import { ipcMain } from 'electron';
import { DataSource } from 'typeorm';
import Room from './database/room';

export default function useDatabase() {
  let datastore: DataSource | null = null;
  let counter = 0;

  ipcMain.handle('connect-to-database', async (event, arg) => {
    console.log(arg);
    datastore = new DataSource({
      type: 'postgres',
      host: arg.host,
      port: arg.port,
      username: arg.username,
      password: arg.password,
      database: 'rinoramud',
      synchronize: true,
      logging: true,
      entities: [Room],
      subscribers: [],
      migrations: [],
    });

    try {
      await datastore.initialize();
    } catch (error) {
      console.log(error);
      return false;
    }

    return true;
  });
}
