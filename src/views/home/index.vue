<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useAppStore } from '@/store/modules/app';

const appStore = useAppStore();

// 定义响应式数据
const data = ref<string>('');

// 调用命令并绑定到 data
invoke<string>('my_custom_command').then(command => {
  data.value = command;
});

// const gap = computed(() => (appStore.isMobile ? 0 : 16));
</script>

<template>
  <NSpace vertical :size="16">
    <NAlert title="my_custom_command" type="info">
      {{ data }}
    </NAlert>

    <NAlert title="appStore" type="info">
      {{ appStore }}
    </NAlert>
  </NSpace>
</template>

<style scoped></style>
