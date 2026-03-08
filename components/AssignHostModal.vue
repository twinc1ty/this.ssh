<template>
  <Modal :is-open="isOpen" @close="closeModal">
    <template #header>
      <div class="assign-host-header">
        <svg
          class="w-6 h-6 mr-3"
          style="color: var(--primary-highlight)"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"
          />
        </svg>
        <h2 class="assign-host-title">
          {{ step === 'select' ? (isEditing ? 'Edit Host' : 'Assign Host') : 'Update Repositories' }}
        </h2>
      </div>
    </template>

    <template #content>
      <div class="assign-host-content">
        <!-- Step 1: Service & Label Selection -->
        <div v-if="step === 'select'">
          <p class="key-info">
            Key: <span class="key-filename">{{ keyFilename }}</span>
          </p>

          <div class="form-group">
            <label class="form-label">Service</label>
            <div class="service-grid">
              <button
                v-for="svc in services"
                :key="svc.id"
                :class="['service-btn', { active: form.service === svc.id }]"
                @click="form.service = svc.id"
              >
                {{ svc.label }}
              </button>
            </div>
          </div>

          <div v-if="form.service === 'custom'" class="form-group">
            <label for="customHost" class="form-label">Hostname</label>
            <input
              id="customHost"
              v-model="form.customHost"
              type="text"
              class="form-input"
              placeholder="e.g. git.mycompany.com"
            />
          </div>

          <div class="form-group">
            <label for="label" class="form-label">Label</label>
            <input
              id="label"
              v-model="form.label"
              type="text"
              class="form-input"
              placeholder="e.g. personal, work, client-x"
              @keyup.enter="assignHost"
            />
            <p class="form-hint">
              A label to identify this key's purpose.
              Your SSH alias will be: <strong class="alias-preview">{{ generatedAlias || '...' }}</strong>
            </p>
          </div>

          <p v-if="message && messageType === 'error'" class="message-error">
            {{ message }}
          </p>
        </div>

        <!-- Step 2: Post-assignment instructions -->
        <div v-if="step === 'done'">
          <p class="message-success done-message">{{ isEditing ? 'Host updated successfully' : 'Host assigned successfully' }}</p>

          <div class="info-box">
            <p class="info-title">Your repositories need updated remote URLs</p>
            <p class="info-text">
              Repos using <strong>{{ actualHostname }}</strong> with this key
              should use the alias instead:
            </p>

            <div class="url-comparison">
              <div class="url-row">
                <span class="url-label">Before:</span>
                <code class="url-value old">git@{{ actualHostname }}:user/repo.git</code>
              </div>
              <div class="url-row">
                <span class="url-label">After:</span>
                <code class="url-value new">git@{{ generatedAlias }}:user/repo.git</code>
                <button class="copy-btn" @click="copyAlias" :title="copyText">
                  {{ copyText }}
                </button>
              </div>
            </div>
          </div>

          <div class="update-section">
            <p class="info-title">Update a repository automatically</p>
            <div class="repo-input-row">
              <input
                v-model="repoPath"
                type="text"
                class="form-input"
                placeholder="Paste the full path to your repo"
                @keyup.enter="updateRemote"
              />
            </div>
            <button
              class="btn-primary update-btn"
              :disabled="!repoPath.trim() || isUpdating"
              @click="updateRemote"
            >
              <span v-if="isUpdating">Updating...</span>
              <span v-else>Update Remote</span>
            </button>
            <p v-if="updateMessage" :class="updateSuccess ? 'message-success' : 'message-error'" class="update-msg">
              {{ updateMessage }}
            </p>
          </div>
        </div>
      </div>
    </template>

    <template #footer>
      <template v-if="step === 'select'">
        <button @click="closeModal" class="btn-secondary">Cancel</button>
        <button
          @click="assignHost"
          :disabled="isAssigning || !canAssign"
          class="btn-primary"
        >
          <span v-if="isAssigning">{{ isEditing ? 'Saving...' : 'Assigning...' }}</span>
          <span v-else>{{ isEditing ? 'Save' : 'Assign' }}</span>
        </button>
      </template>
      <template v-else>
        <button @click="closeModal" class="btn-primary">Done</button>
      </template>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import type { HostEntry } from "~/types/core-types";
import Modal from "./Modal.vue";

interface Props {
  isOpen: boolean;
  keyFilename: string;
  editingHost?: HostEntry | null;
}

interface Emits {
  (e: "close"): void;
  (e: "host-assigned"): void;
}

const props = withDefaults(defineProps<Props>(), {
  editingHost: null,
});
const emit = defineEmits<Emits>();

const services = [
  { id: "github", label: "GitHub", hostname: "github.com", user: "git" },
  { id: "gitlab", label: "GitLab", hostname: "gitlab.com", user: "git" },
  { id: "bitbucket", label: "Bitbucket", hostname: "bitbucket.org", user: "git" },
  { id: "custom", label: "Custom", hostname: "", user: "" },
];

const step = ref<"select" | "done">("select");
const isAssigning = ref(false);
const isUpdating = ref(false);
const message = ref("");
const messageType = ref<"success" | "error">("success");
const copyText = ref("Copy");
const repoPath = ref("");
const updateMessage = ref("");
const updateSuccess = ref(false);
const oldHostAlias = ref("");

const form = ref({
  service: "github",
  label: "",
  customHost: "",
});

const isEditing = computed(() => !!props.editingHost);

// Reverse-engineer service and label from an existing HostEntry
const populateFromHostEntry = (entry: HostEntry) => {
  oldHostAlias.value = entry.host;

  // Detect service from hostname
  const matchedService = services.find(
    (s) => s.id !== "custom" && s.hostname === entry.hostname
  );

  if (matchedService) {
    form.value.service = matchedService.id;
    // Extract label: alias is "service-label", so strip the service prefix
    const prefix = matchedService.id + "-";
    form.value.label = entry.host.startsWith(prefix)
      ? entry.host.slice(prefix.length)
      : entry.host;
  } else {
    form.value.service = "custom";
    form.value.customHost = entry.hostname || "";
    // Extract label: alias is "hostprefix-label"
    const hostPrefix = (entry.hostname || "").split(".")[0];
    const prefix = hostPrefix + "-";
    form.value.label = entry.host.startsWith(prefix)
      ? entry.host.slice(prefix.length)
      : entry.host;
  }
};

// Watch for modal open with editing data
watch(() => props.isOpen, (open) => {
  if (open && props.editingHost) {
    populateFromHostEntry(props.editingHost);
  }
});

const selectedService = computed(() =>
  services.find((s) => s.id === form.value.service)
);

const actualHostname = computed(() => {
  if (form.value.service === "custom") return form.value.customHost.trim();
  return selectedService.value?.hostname || "";
});

const generatedAlias = computed(() => {
  const label = form.value.label.trim().toLowerCase().replace(/\s+/g, "-");
  if (!label) return "";
  if (form.value.service === "custom") {
    const host = form.value.customHost.trim().split(".")[0] || "host";
    return `${host}-${label}`;
  }
  return `${form.value.service}-${label}`;
});

const canAssign = computed(() => {
  if (!form.value.label.trim()) return false;
  if (form.value.service === "custom" && !form.value.customHost.trim()) return false;
  return true;
});

const closeModal = () => {
  form.value = { service: "github", label: "", customHost: "" };
  step.value = "select";
  message.value = "";
  repoPath.value = "";
  updateMessage.value = "";
  oldHostAlias.value = "";
  emit("close");
};

const assignHost = async () => {
  if (!canAssign.value) return;

  isAssigning.value = true;
  message.value = "";

  const alias = generatedAlias.value;
  const hostname = actualHostname.value;
  const user = selectedService.value?.user || null;

  try {
    // If editing and the alias changed, remove the old entry first
    if (isEditing.value && oldHostAlias.value && oldHostAlias.value !== alias) {
      await invoke<string>("remove_host_entry", { host: oldHostAlias.value });
    }

    await invoke<string>("assign_host_to_key", {
      host: alias,
      hostname,
      user,
      keyFilename: props.keyFilename,
    });

    emit("host-assigned");
    step.value = "done";
  } catch (error: any) {
    message.value = error.toString();
    messageType.value = "error";
  } finally {
    isAssigning.value = false;
  }
};

const copyAlias = () => {
  const text = `git@${generatedAlias.value}:`;
  navigator.clipboard.writeText(text).catch((err) => {
    console.error("Failed to copy:", err);
  });
  copyText.value = "Copied!";
  setTimeout(() => {
    copyText.value = "Copy";
  }, 2000);
};

const updateRemote = async () => {
  if (!repoPath.value.trim()) return;

  isUpdating.value = true;
  updateMessage.value = "";

  try {
    const result = await invoke<string>("update_git_remote", {
      repoPath: repoPath.value.trim(),
      oldHost: actualHostname.value,
      newHost: generatedAlias.value,
    });
    updateMessage.value = result;
    updateSuccess.value = true;
  } catch (error: any) {
    updateMessage.value = error.toString();
    updateSuccess.value = false;
  } finally {
    isUpdating.value = false;
  }
};
</script>

<style scoped>
.assign-host-header {
  display: flex;
  align-items: center;
}

.assign-host-title {
  color: var(--primary-highlight);
  font-size: 1.25rem;
  font-weight: 600;
  margin: 0;
  font-family: "Righteous", sans-serif;
}

.assign-host-content {
  color: var(--primary-highlight);
}

.key-info {
  font-size: 0.875rem;
  color: var(--secondary-highlight);
  margin-bottom: 1.25rem;
}

.key-filename {
  font-family: monospace;
  color: var(--primary-highlight);
  font-weight: 500;
}

.form-group {
  margin-bottom: 1rem;
}

.form-label {
  display: block;
  font-weight: 600;
  font-size: 0.875rem;
  margin-bottom: 0.375rem;
  color: var(--primary-highlight);
}

.service-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.5rem;
}

.service-btn {
  padding: 0.5rem;
  background-color: var(--secondary-bg);
  border: 1px solid var(--secondary-highlight);
  border-radius: 0.375rem;
  color: var(--secondary-highlight);
  font-size: 0.875rem;
  font-family: "Questrial", sans-serif;
  cursor: pointer;
  transition: all 0.2s ease;
}

.service-btn:hover {
  border-color: var(--primary-highlight);
  color: var(--primary-highlight);
}

.service-btn.active {
  border-color: var(--primary-highlight);
  color: var(--primary-highlight);
  background-color: var(--primary-bg);
  font-weight: 600;
}

.form-input {
  width: 100%;
  padding: 0.5rem 0.75rem;
  background-color: var(--secondary-bg);
  border: 1px solid var(--secondary-highlight);
  border-radius: 0.375rem;
  color: var(--primary-highlight);
  font-size: 0.875rem;
  font-family: "Questrial", sans-serif;
  outline: none;
  transition: border-color 0.2s ease;
  box-sizing: border-box;
}

.form-input:focus {
  border-color: var(--primary-highlight);
}

.form-input::placeholder {
  color: var(--secondary-highlight);
  opacity: 0.6;
}

.form-hint {
  font-size: 0.75rem;
  color: var(--secondary-highlight);
  margin-top: 0.25rem;
  opacity: 0.8;
}

.alias-preview {
  color: var(--primary-highlight);
  font-family: monospace;
}

.done-message {
  text-align: center;
  font-weight: 600;
  margin-bottom: 1rem;
}

.info-box {
  background-color: var(--secondary-bg);
  border: 1px solid var(--secondary-highlight);
  border-radius: 0.5rem;
  padding: 1rem;
  margin-bottom: 1rem;
}

.info-title {
  font-weight: 600;
  font-size: 0.875rem;
  margin-bottom: 0.5rem;
}

.info-text {
  font-size: 0.8rem;
  color: var(--secondary-highlight);
  margin-bottom: 0.75rem;
}

.url-comparison {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.url-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.url-label {
  font-size: 0.75rem;
  color: var(--secondary-highlight);
  min-width: 3rem;
}

.url-value {
  font-size: 0.75rem;
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
  word-break: break-all;
}

.url-value.old {
  background-color: rgba(239, 68, 68, 0.15);
  color: #f87171;
  text-decoration: line-through;
}

.url-value.new {
  background-color: rgba(34, 197, 94, 0.15);
  color: #4ade80;
}

.copy-btn {
  background: none;
  border: 1px solid var(--secondary-highlight);
  border-radius: 0.25rem;
  color: var(--secondary-highlight);
  font-size: 0.7rem;
  padding: 0.125rem 0.375rem;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: "Questrial", sans-serif;
}

.copy-btn:hover {
  color: var(--primary-highlight);
  border-color: var(--primary-highlight);
}

.update-section {
  background-color: var(--secondary-bg);
  border: 1px solid var(--secondary-highlight);
  border-radius: 0.5rem;
  padding: 1rem;
}

.repo-input-row {
  margin-bottom: 0.5rem;
}

.update-btn {
  width: 100%;
  margin-top: 0.25rem;
}

.update-msg {
  margin-top: 0.5rem;
  font-size: 0.8rem;
}

.message-error {
  color: #ef4444;
  font-size: 0.875rem;
  margin-top: 0.5rem;
}

.message-success {
  color: #22c55e;
  font-size: 0.875rem;
  margin-top: 0.5rem;
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
  opacity: 0.9;
  transform: translateY(-1px);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
}
</style>
