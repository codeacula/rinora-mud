/* eslint-disable no-console */
import { ipcMain } from 'electron';
import knex, { Knex } from 'knex';

export default function useDatabase() {
  let datastore: Knex | null = null;

  ipcMain.handle('connect-to-database', async (event, arg) => {
    try {
      if (datastore) {
        console.log('Already connected, ignoring request.');
        return true;
      }
      datastore = knex({
        client: 'pg',
        connection: {
          host: arg.host,
          port: arg.port,
          user: arg.username,
          database: 'rinoramud',
          password: arg.password,
          ssl: false,
        },
      });

      // Check the connection
      await datastore.raw('SELECT * FROM users');
      return true;
    } catch (error) {
      console.log(error);
      datastore = null;
      return false;
    }
  });
}
