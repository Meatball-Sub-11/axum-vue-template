<script setup lang="ts">
import { ref, onMounted } from 'vue';

const backendMessage = ref('Connecting to backend...');

onMounted(() => {
  fetch('/api/health')
    .then(response => {
      if (!response.ok) {
        throw new Error('Network response was not ok');
      }
      return response.json();
    })
    .then(data => {
      backendMessage.value = data.message;
    })
    .catch(error => {
      console.error('Error fetching from backend:', error);
      backendMessage.value = 'Failed to connect to the backend.';
    });
});
</script>

<template>
  <div class="container">
    <h1>Axum + Vue Template</h1>
    <div class="status-box">
      <p>Backend Status:</p>
      <p class="status-message">{{ backendMessage }}</p>
    </div>
  </div>
</template>

<style scoped>
.container {
  max-width: 600px;
  margin: 4rem auto;
  padding: 2rem;
  font-family: sans-serif;
  text-align: center;
}
.status-box {
  margin-top: 2rem;
  padding: 1.5rem;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  background-color: #f7fafc;
}
.status-message {
  font-weight: bold;
  font-size: 1.2rem;
  color: #2d3748;
  margin-top: 0.5rem;
}
</style>
