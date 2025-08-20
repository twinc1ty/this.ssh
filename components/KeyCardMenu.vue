<template>
  <div class="flex justify-around items-center">
    <button class="my-2 w-[90%] p-1 m-1" @click="copyPublicKey()">
      <p>{{ copyButtonCTA }}</p>
      <div :class="triggerAnimationIndex.copy ? loadingBarClass : ''" />
    </button>
    <button class="my-2 w-[90%] p-1 m-1" @click="removeKey()">
      <p>Remove</p>
      <div :class="triggerAnimationIndex.remove ? loadingBarClass : ''" />
    </button>
  </div>
</template>

<script setup lang="ts">
import type { TriggerKeyMenuAnimationIndex } from "~/types/core-types";

defineProps<{
  publicKey: string;
}>();

const emit = defineEmits<{
  (e: "copy"): void;
  (e: "remove"): void;
}>();

const copyButtonCTA = ref("Copy Public Key");
const triggerAnimationIndex = ref<TriggerKeyMenuAnimationIndex>({
  copy: false,
  remove: false,
});

const loadingBarClass = "loading-bar";

const copyPublicKey = () => {
  emit("copy");
  copyButtonCTA.value = "Copied!";
  triggerAnimationIndex.value = { copy: true, remove: false };
  setTimeout(() => {
    copyButtonCTA.value = "Copy Public Key";
    triggerAnimationIndex.value = { copy: false, remove: false };
  }, 2000);
};
const removeKey = () => {
  emit("remove");
  triggerAnimationIndex.value = { copy: false, remove: true };
  setTimeout(() => {
    triggerAnimationIndex.value = { copy: false, remove: false };
  }, 2000);
};
</script>
<style scoped>
.loading-bar {
  width: 100%;
  height: 2px;
  background-color: var(--primary-highlight);
  animation: loading-bar-animation 2s infinite;
}

@keyframes loading-bar-animation {
  0% {
    width: 0;
    opacity: 0;
  }
  50% {
    width: 100%;
    opacity: 1;
    background-color: var(--primary-bg);
  }
  100% {
    width: 0;
    opacity: 0;
  }
}
</style>
