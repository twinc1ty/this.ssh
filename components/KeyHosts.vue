<template>
  <div class="key-hosts">
    <div class="hosts-row">
      <div v-if="hosts.length > 0" class="hosts-badges">
        <HostBadge
          v-for="entry in hosts"
          :key="entry.host"
          :host="entry.host"
          @remove="removeHost(entry.host)"
          @edit="editHost(entry)"
        />
      </div>
      <button class="assign-btn" @click.stop="showAssignModal">
        + Assign Host
      </button>
    </div>
  </div>

  <AssignHostModal
    :is-open="isAssignModalOpen"
    :key-filename="keyFilename"
    :editing-host="editingEntry"
    @close="closeModal"
    @host-assigned="$emit('host-assigned')"
  />
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import type { HostEntry } from "~/types/core-types";
import HostBadge from "./HostBadge.vue";
import AssignHostModal from "./AssignHostModal.vue";

interface Props {
  hosts: HostEntry[];
  keyFilename: string;
}

interface Emits {
  (e: "host-removed"): void;
  (e: "host-assigned"): void;
}

defineProps<Props>();
const emit = defineEmits<Emits>();

const isAssignModalOpen = ref(false);
const editingEntry = ref<HostEntry | null>(null);

const showAssignModal = () => {
  editingEntry.value = null;
  isAssignModalOpen.value = true;
};

const editHost = (entry: HostEntry) => {
  editingEntry.value = entry;
  isAssignModalOpen.value = true;
};

const closeModal = () => {
  isAssignModalOpen.value = false;
  editingEntry.value = null;
};

const removeHost = async (host: string) => {
  try {
    await invoke<string>("remove_host_entry", { host });
    emit("host-removed");
  } catch (error: any) {
    console.error("Failed to remove host:", error);
  }
};
</script>

<style scoped>
.key-hosts {
  margin-top: 0.375rem;
}

.hosts-row {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 0.375rem;
}

.hosts-badges {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
}

.assign-btn {
  background: none;
  border: 1px dashed var(--secondary-highlight);
  border-radius: 9999px;
  color: var(--secondary-highlight);
  font-size: 0.7rem;
  padding: 0.125rem 0.5rem;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: "Questrial", sans-serif;
}

.assign-btn:hover {
  color: var(--primary-highlight);
  border-color: var(--primary-highlight);
}
</style>
