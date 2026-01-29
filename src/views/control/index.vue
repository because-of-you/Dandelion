<script setup lang="ts">
const data = 'hello world';
</script>

<template>
  <NSpace vertical>
    <NAlert :title="data" type="info">
      {{ data }}
    </NAlert>
  </NSpace>
</template>

<style scoped></style>
