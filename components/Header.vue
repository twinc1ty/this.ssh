<template>
  <div class="relative w-full">
    <!-- Status Bar -->
    <div
      class="fixed top-0 left-0 w-full h-4 z-50"
      :class="isSshAgentRunning ? 'bg-green-500' : 'bg-red-500'"
    >
      <p class="text-center text-white text-xs">
        {{ isSshAgentRunning ? "SSH Agent Running" : "SSH Agent Not Running" }}
      </p>
    </div>
    <div class="header-container mt-[6px]">
      <!-- Status Strip -->
      <p
        class="text-2xl font-bold theme-heading-regular header-headline"
        @click="toggleDebugMode"
      >
        <i>this</i>.ssh
      </p>
      <div class="sub-heading flex justify-center items-center">
        <p class="font-bold theme-text-regular sub-text">
          SSH management made simple
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  isSshAgentRunning: boolean;
}>();

interface Emits {
  (e: "toggleDebugMode"): boolean;
}

const emit = defineEmits<Emits>();
const isDebugMode = ref<boolean>(false);

const toggleDebugMode = () => {
  emit("toggleDebugMode");
};
</script>

<style scoped>
.header-container {
  background-color: var(--primary-highlight);
  padding: 10px;
  display: flex;
  justify-content: end;
  flex-direction: column;
  align-items: end;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.header-headline {
  color: var(--secondary-bg);
  letter-spacing: 0.1em;
  font-size: 50px;
  margin-top: 10px;
  transition: all 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.sub-text {
  color: var(--secondary-highlight);
}

.header-headline:hover {
  cursor: pointer;
  animation: header-headline-hover 0.5s;
}

@keyframes header-headline-hover {
  0% {
    letter-spacing: 0.1em;
  }
  25% {
    letter-spacing: 0.2.5em;
    color: #575656;
  }
  50% {
    letter-spacing: 0.2em;
  }
  100% {
    letter-spacing: 0.1em;
  }
}
</style>
