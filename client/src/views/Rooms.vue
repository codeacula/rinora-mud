<script async setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref, onMounted, ref } from "vue";

interface Room {
  name: string;
}

let rooms: Ref<Room[]> = ref([]);

onMounted(async () => {
  try {
    rooms.value = await invoke<Room[]>("get_rooms");
    console.log("Here", rooms);
  } catch (error: any) {
    console.log(error);
  }
});
</script>

<template>
  <div class="rooms">
    <h1>Rooms</h1>
    <ul>
      <li v-for="room in rooms" :key="room.name">
        {{ room.name }}
      </li>
    </ul>
  </div>
</template>

<style scoped></style>
