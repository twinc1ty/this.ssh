<template>
  <Modal :is-open="isOpen" @close="closeModal">
    <!-- Header Slot -->
    <template #header>
      <div class="remove-key-header">
        <svg
          class="w-6 h-6 text-red-500 mr-3"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"
          />
        </svg>
        <h2 class="remove-key-title">Remove SSH Key</h2>
      </div>
    </template>

    <!-- Content Slot -->
    <template #content>
      <div class="remove-key-content">
        <p class="warning-text">
          Are you sure you want to remove this SSH key?
        </p>

        <div v-if="keyDetails" class="key-details">
          <div class="key-detail-item">
            <span class="detail-label">Key Type:</span>
            <span class="detail-value">{{ keyDetails.keyType }}</span>
          </div>
          <div class="key-detail-item">
            <span class="detail-label">Email:</span>
            <span class="detail-value">{{ keyDetails.email }}</span>
          </div>
          <div class="key-detail-item">
            <span class="detail-label">Fingerprint:</span>
            <span class="detail-value fingerprint">{{
              keyDetails.keyPid
            }}</span>
          </div>
          <div class="key-detail-item">
            <span class="detail-label">Filename:</span>
            <span class="detail-value filename">{{ keyDetails.filename }}</span>
          </div>
        </div>

        <div class="warning-box">
          <p class="warning-message">
            <strong>⚠️ Warning:</strong> This action will permanently delete the
            SSH key files from your system.
          </p>
          <ul class="warning-list">
            <li>You will lose access to any servers using this key</li>
            <li>This action cannot be undone</li>
            <li>Make sure you have a backup if needed</li>
          </ul>
        </div>
      </div>
    </template>

    <!-- Footer Slot -->
    <template #footer>
      <button @click="closeModal" class="btn-secondary">Cancel</button>
      <button @click="confirmRemove" :disabled="isRemoving" class="btn-danger">
        <span v-if="isRemoving">Removing...</span>
        <span v-else>Remove Key</span>
      </button>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import Modal from "./Modal.vue";
import type { Key } from "~/types/core-types";

interface Props {
  isOpen: boolean;
  keyDetails: Key | null;
}

interface Emits {
  (e: "close"): void;
  (e: "key-removed"): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const isRemoving = ref(false);

const closeModal = () => {
  emit("close");
};

const confirmRemove = async () => {
  if (!props.keyDetails) return;

  isRemoving.value = true;

  try {
    // Using the filename field for key removal
    const keyFilename = props.keyDetails.filename;

    const result = await invoke<string>("remove_ssh_key", {
      keyFilename: keyFilename,
    });

    // Emit success and close modal
    emit("key-removed");
    closeModal();
  } catch (error: any) {
    console.error("Failed to remove SSH key:", error);
  } finally {
    isRemoving.value = false;
  }
};
</script>

<style scoped>
.remove-key-header {
  display: flex;
  align-items: center;
}

.remove-key-title {
  color: var(--primary-highlight);
  font-size: 1.25rem;
  font-weight: 600;
  margin: 0;
  font-family: "Righteous", sans-serif;
}

.remove-key-content {
  color: var(--primary-highlight);
}

.warning-text {
  font-size: 1.1rem;
  font-weight: 500;
  margin-bottom: 1.5rem;
  text-align: center;
}

.key-details {
  background-color: var(--secondary-bg);
  border: 1px solid var(--secondary-highlight);
  border-radius: 0.5rem;
  padding: 1rem;
  margin-bottom: 1.5rem;
}

.key-detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.key-detail-item:last-child {
  margin-bottom: 0;
}

.detail-label {
  font-weight: 600;
  color: var(--secondary-highlight);
}

.detail-value {
  font-weight: 400;
  color: var(--primary-highlight);
}

.fingerprint {
  font-family: monospace;
  font-size: 0.875rem;
  word-break: break-all;
}

.filename {
  font-family: monospace;
  font-size: 0.875rem;
  color: var(--secondary-highlight);
  font-weight: 500;
}

.warning-box {
  background-color: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.3);
  border-radius: 0.5rem;
  padding: 1rem;
}

.warning-message {
  color: #ef4444;
  font-weight: 500;
  margin-bottom: 0.75rem;
}

.warning-list {
  color: #ef4444;
  margin: 0;
  padding-left: 1.25rem;
}

.warning-list li {
  margin-bottom: 0.25rem;
  font-size: 0.875rem;
}

.btn-secondary {
  background-color: var(--primary-bg);
  color: var(--primary-highlight);
  border: 2px solid var(--secondary-highlight);
  padding: 0.5rem 1rem;
  border-radius: 0.375rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: "Questrial", sans-serif;
}

.btn-secondary:hover {
  background-color: var(--secondary-highlight);
  color: var(--primary-bg);
  border-color: var(--primary-highlight);
}

.btn-danger {
  background-color: #ef4444;
  color: white;
  border: 2px solid #ef4444;
  padding: 0.5rem 1rem;
  border-radius: 0.375rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: "Questrial", sans-serif;
}

.btn-danger:hover:not(:disabled) {
  background-color: #dc2626;
  border-color: #dc2626;
  transform: translateY(-1px);
}

.btn-danger:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
}
</style>
