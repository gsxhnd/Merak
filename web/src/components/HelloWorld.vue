<script setup lang="ts">
// import { ref } from "vue";
import Button from "primevue/button";
import { userLogin } from "@/api/user";
import { onMounted } from "vue";

onMounted(async () => {
  await userLogin().then((v) => {
    console.log(v);
  });
});

defineProps<{ msg: string }>();
</script>

<template>
  <h1>{{ msg }}</h1>
  <Button label="Submit" />
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
