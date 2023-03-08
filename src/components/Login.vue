<script setup lang="ts">

import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const loginMsg = ref("");
const username = ref("");
const password = ref("");

async function login() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  loginMsg.value = await invoke("login", { username: username.value, password: password.value });
}
</script>

<template>
  <div class="content">
    <form>
      <h2>Login</h2>
      <div class="input">
        <label for="username">Username: </label>
        <input v-model="username"
          class="form-control"
          type="text"
          name="username"
          placeholder="username"
        />
      </div>
      <div class="input">
        <label for="password">Password: </label>
        <input v-model="password"
          class="form-control"
          type="password"
          name="password"
          placeholder="********"
        />
      </div>
      <button type="button" @click="login()">Login</button>
    </form>
    <p>{{ loginMsg }}</p>
  </div>
</template>
