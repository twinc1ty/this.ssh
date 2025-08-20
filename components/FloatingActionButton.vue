<template>
  <div class="fab-container">
    <button
      @click="toggleFab"
      class="fab-main"
      :class="{ 'fab-expanded': isExpanded }"
      aria-label="Toggle actions"
    >
      <svg
        v-if="!isExpanded"
        class="w-6 h-6"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M12 6v6m0 0v6m0-6h6m-6 0H6"
        />
      </svg>
      <svg
        v-else
        class="w-6 h-6"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M6 18L18 6M6 6l12 12"
        />
      </svg>
    </button>

    <!-- FAB Actions -->
    <div class="fab-actions" :class="{ 'fab-actions-visible': isExpanded }">
      <!-- Create SSH Key Action -->
      <button
        @click="openCreateKeyModal"
        class="fab-action fab-action-create"
        title="Create New SSH Key"
      >
        <svg
          class="w-5 h-5"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 6v6m0 0v6m0-6h6m-6 0H6"
          />
        </svg>
        <span class="fab-action-label">Create Key</span>
      </button>
    </div>

    <!-- Create Key Modal -->
    <CreateKeyModal
      :is-open="isCreateKeyModalOpen"
      @close="closeCreateKeyModal"
      @key-created="handleKeyCreated"
    />
  </div>
</template>

<script setup lang="ts">
import CreateKeyModal from "./CreateKeyModal.vue";

interface Emits {
  (e: "key-created"): void;
}

const emit = defineEmits<Emits>();

const isExpanded = ref(false);
const isCreateKeyModalOpen = ref(false);

const toggleFab = () => {
  isExpanded.value = !isExpanded.value;
};

const openCreateKeyModal = () => {
  isCreateKeyModalOpen.value = true;
  isExpanded.value = false; // Close FAB when opening modal
};

const closeCreateKeyModal = () => {
  isCreateKeyModalOpen.value = false;
};

const handleKeyCreated = () => {
  emit("key-created");
};
</script>

<style scoped>
.fab-container {
  position: fixed;
  bottom: 1rem;
  right: 1rem;
  z-index: 1000;
}

.fab-main {
  width: 3.5rem;
  height: 3.5rem;
  border-radius: 50%;
  background-color: var(--primary-highlight);
  color: var(--primary-bg);
  border: none;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1001;
}

.fab-main:hover {
  background-color: var(--secondary-highlight);
  transform: scale(1.1);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.4);
}

.fab-main:active {
  transform: scale(0.95);
}

.fab-main.fab-expanded {
  background-color: var(--secondary-highlight);
}

.fab-actions {
  position: absolute;
  bottom: 0.5rem;
  right: 4rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  opacity: 0;
  visibility: hidden;
  transform: translateY(1rem);
  transition: all 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.fab-actions-visible {
  opacity: 1;
  visibility: visible;
  transform: translateY(0);
}

.fab-action {
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 50%;
  background-color: var(--secondary-bg);
  color: var(--primary-highlight);
  border: 2px solid var(--primary-highlight);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
}

.fab-action:hover {
  background-color: var(--primary-highlight);
  color: var(--primary-bg);
  transform: scale(1.1);
}

.fab-action:active {
  transform: scale(0.95);
}

.fab-action-label {
  position: absolute;
  right: 3rem;
  background-color: var(--secondary-bg);
  color: var(--primary-highlight);
  padding: 0.5rem 0.75rem;
  border-radius: 0.5rem;
  font-size: 0.875rem;
  white-space: nowrap;
  opacity: 0;
  visibility: hidden;
  transform: translateX(0.5rem);
  transition: all 0.3s ease;
  border: 1px solid var(--primary-highlight);
}

.fab-action:hover .fab-action-label {
  opacity: 1;
  visibility: visible;
  transform: translateX(0);
}

/* Animation delays for staggered appearance */
.fab-action:nth-child(1) {
  transition-delay: 0.1s;
}

.fab-actions-visible .fab-action:nth-child(1) {
  transition-delay: 0s;
}
</style>
