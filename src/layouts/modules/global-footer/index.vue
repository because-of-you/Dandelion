<script setup lang="ts">
import { $t } from '@/locales';

defineOptions({
  name: 'GlobalFooter'
});
const fn_11 = () => {
  // window.$notification?.success({
  //   title: '2222',
  //   content: '他们说往前看',
  //   duration: 3_000
  // });
  window.$message?.info($t(`common.footer.message`));
};
</script>

<template>
  <DarkModeContainer class="h-full flex-center">
    <div @mouseover="fn_11">Copyright MIT © 2026 because-of-you</div>
  </DarkModeContainer>
</template>

<style scoped></style>
