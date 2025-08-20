<template>
  <div
    v-if="isOpen"
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
  >
    <div class="modal-container">
      <div class="modal-header">
        <p class="modal-title theme-text-regular text-xl">Create New SSH Key</p>
        <button @click="closeModal" class="modal-close-btn">
          <svg
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
            ></path>
          </svg>
        </button>
      </div>

      <form @submit.prevent="createKey" class="modal-form">
        <div class="form-group">
          <label for="email" class="form-label"> Email Address </label>
          <input
            id="email"
            v-model="form.email"
            type="email"
            required
            class="form-input"
            placeholder="your.email@example.com"
          />
        </div>

        <div class="form-group">
          <label for="keyType" class="form-label"> Key Type </label>
          <select
            id="keyType"
            v-model="form.keyType"
            required
            class="form-select"
          >
            <option value="rsa">RSA</option>
            <option value="ed25519">Ed25519</option>
            <option value="ecdsa">ECDSA</option>
          </select>
        </div>

        <div v-if="form.keyType === 'rsa'" class="form-group">
          <label for="keySize" class="form-label"> Key Size (bits) </label>
          <select
            id="keySize"
            v-model="form.keySize"
            required
            class="form-select"
          >
            <option value="2048">2048</option>
            <option value="4096">4096</option>
          </select>
        </div>

        <div v-if="form.keyType === 'ecdsa'" class="form-group">
          <label for="keySize" class="form-label"> Key Size (bits) </label>
          <select
            id="keySize"
            v-model="form.keySize"
            required
            class="form-select"
          >
            <option value="256">256</option>
            <option value="384">384</option>
            <option value="521">521</option>
          </select>
        </div>

        <div class="form-group">
          <label for="passphrase" class="form-label">
            Passphrase (optional)
          </label>
          <input
            id="passphrase"
            v-model="form.passphrase"
            type="password"
            class="form-input"
            placeholder="Leave empty for no passphrase"
          />
        </div>

        <div class="modal-actions">
          <button type="button" @click="closeModal" class="">
            <p>Cancel</p>
          </button>
          <button type="submit" :disabled="isCreating" class="">
            <p v-if="isCreating">Creating...</p>
            <p v-else>Create Key</p>
          </button>
        </div>
      </form>

      <!-- Success/Error Messages -->
      <div
        v-if="message"
        class="message-container"
        :class="messageType === 'success' ? 'message-success' : 'message-error'"
      >
        {{ message }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

interface Props {
  isOpen: boolean;
}

interface Emits {
  (e: "close"): void;
  (e: "key-created"): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const form = ref({
  email: "",
  keyType: "ed25519",
  keySize: 256,
  passphrase: "",
});

const isCreating = ref(false);
const message = ref("");
const messageType = ref<"success" | "error">("success");

const closeModal = () => {
  emit("close");
  resetForm();
};

const resetForm = () => {
  form.value = {
    email: "",
    keyType: "ed25519",
    keySize: 256,
    passphrase: "",
  };
  message.value = "";
};

const createKey = async () => {
  if (!form.value.email.trim()) {
    message.value = "Please enter an email address";
    messageType.value = "error";
    return;
  }

  isCreating.value = true;
  message.value = "";

  try {
    const result = await invoke<string>("create_ssh_key", {
      email: form.value.email.trim(),
      keyType: form.value.keyType,
      keySize: form.value.keySize,
      passphrase: form.value.passphrase.trim() || null,
    });

    message.value = result;
    messageType.value = "success";

    // timer to close modal and refresh keys
    setTimeout(() => {
      emit("key-created");
      closeModal();
    }, 2000);
  } catch (error: any) {
    message.value = error || "Failed to create SSH key";
    messageType.value = "error";
  } finally {
    isCreating.value = false;
  }
};

// watching keyType to adjust default keySize
watch(
  () => form.value.keyType,
  (newType) => {
    if (newType === "rsa") {
      form.value.keySize = 2048;
    } else if (newType === "ecdsa") {
      form.value.keySize = 256;
    } else if (newType === "ed25519") {
      form.value.keySize = 256;
    }
  }
);
</script>

<style scoped>
.modal-container {
  background-color: var(--primary-bg);
  border: 2px solid var(--primary-highlight);
  border-radius: 0.75rem;
  padding: 1.5rem;
  width: 24rem;
  max-width: 90vw;
  margin: 1rem;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.modal-title {
  color: var(--primary-highlight);
}

.modal-close-btn {
  background: none;
  border: none;
  color: var(--secondary-highlight);
  cursor: pointer;
  padding: 0.25rem;
  border-radius: 0.25rem;
  transition: all 0.2s ease;
}

.modal-close-btn:hover {
  color: var(--primary-highlight);
  background-color: var(--primary-bg);
}

.modal-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-group {
  display: flex;
  flex-direction: column;
}

.form-label {
  display: block;
  font-size: 0.875rem;
  font-weight: 200;
  color: var(--primary-highlight);
  margin-bottom: 0.25rem;
  font-family: "Questrial", sans-serif;
}

.form-input,
.form-select {
  width: 100%;
  padding: 0.5rem 0.75rem;
  border: 2px solid var(--secondary-highlight);
  border-radius: 0.375rem;
  background-color: var(--primary-bg);
  color: var(--primary-highlight);
  font-family: "Questrial", sans-serif;
  transition: all 0.2s ease;
}

.form-input:focus,
.form-select:focus {
  outline: none;
  border-color: var(--primary-highlight);
  box-shadow: 0 0 0 3px rgba(223, 208, 184, 0.1);
}

.form-input::placeholder {
  color: var(--secondary-highlight);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding-top: 1rem;
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

.btn-primary {
  background-color: var(--primary-highlight);
  color: var(--primary-bg);
  border: 2px solid var(--primary-highlight);
  padding: 0.5rem 1rem;
  border-radius: 0.375rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: "Questrial", sans-serif;
}

.btn-primary:hover:not(:disabled) {
  background-color: var(--secondary-highlight);
  border-color: var(--secondary-highlight);
  transform: translateY(-1px);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
}

.message-container {
  margin-top: 1rem;
  padding: 0.75rem;
  border-radius: 0.375rem;
  font-family: "Questrial", sans-serif;
  font-weight: 500;
}

.message-success {
  background-color: rgba(34, 197, 94, 0.1);
  color: #22c55e;
  border: 1px solid rgba(34, 197, 94, 0.2);
}

.message-error {
  background-color: rgba(239, 68, 68, 0.1);
  color: #ef4444;
  border: 1px solid rgba(239, 68, 68, 0.2);
}
</style>
