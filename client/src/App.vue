<script setup lang="ts">
import Sidebar from "./components/Sidebar.vue";
</script>

<template>
  <main class="flex prose max-w-none">
    <div class="flex-initial w-64">
      <Sidebar />
    </div>
    <div class="flex-auto">
      <router-view></router-view>
    </div>
  </main>
</template>

<style scoped></style>
