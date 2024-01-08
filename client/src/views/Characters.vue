<script async setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted } from "vue";

interface Character {
  name: string;
}

let characters: Character[] = [];

onMounted(async () => {
  try {
    characters = await invoke<Character[]>("get_characters");
    console.log("Here", characters);
  } catch (error: any) {
    console.log(error);
  }
});
</script>

<template>
  <div class="characters">
    <h1>Characters</h1>
    <ul>
      <li v-for="character in characters" :key="character.name">
        {{ character.name }}
      </li>
    </ul>
  </div>
</template>

<style scoped></style>
