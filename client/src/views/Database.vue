<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { inject, ref } from "vue";
import router from "../router";
import { Authentication } from "../composables/useAuthentication";

interface ConnectionResult {
  success: boolean;
  error: string | null;
}

const connectionError = ref<string | null>(null);
const host = ref("");
const port = ref("5432");
const username = ref("");
const password = ref("");

const authentication = inject<Authentication>("authentication");

if (authentication === undefined) {
  throw new Error("authentication is undefined");
}

const connect = async (event: Event) => {
  event.preventDefault();

  try {
    let conn_result = await invoke<ConnectionResult>("connect_to_database", {
      connSettings: {
        host: host.value,
        port: port.value,
        username: username.value,
        password: password.value,
      },
    });

    if (conn_result.success) {
      authentication.updateIsAuthenticated(true);
      router.push("/");
    } else {
      connectionError.value =
        "Failed to connect to database:" + conn_result.error;
    }
  } catch (error: any) {
    console.log(error);
    connectionError.value = "Failed to connect to database:" + error.message;
  }
};
</script>

<template>
  <div class="database">
    <h1>Database</h1>
    <form @submit="connect">
      <p v-if="connectionError">{{ connectionError }}</p>
      <p>
        <label>Host: <input type="text" v-model="host" /></label>
      </p>
      <p>
        <label>Port: <input type="text" v-model="port" /></label>
      </p>
      <p>
        <label>Username: <input type="text" v-model="username" /></label>
      </p>
      <p>
        <label>Password: <input type="password" v-model="password" /></label>
      </p>
      <p>
        <button type="submit">Connect</button>
      </p>
    </form>
  </div>
</template>

<style scoped></style>
