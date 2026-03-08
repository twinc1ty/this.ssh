<template>
  <span class="host-badge">
    <span class="host-name" @click.stop="$emit('edit')" role="button" title="Edit host">{{ host === '*' ? 'default' : host }}</span>
    <button
      v-if="removable"
      class="host-remove-btn"
      @click.stop="$emit('remove')"
      aria-label="Remove host"
    >
      <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>
  </span>
</template>

<script setup lang="ts">
interface Props {
  host: string;
  removable?: boolean;
}

interface Emits {
  (e: "remove"): void;
  (e: "edit"): void;
}

withDefaults(defineProps<Props>(), {
  removable: true,
});
defineEmits<Emits>();
</script>

<style scoped>
.host-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  background-color: var(--primary-bg);
  border: 1px solid var(--secondary-highlight);
  border-radius: 9999px;
  padding: 0.125rem 0.5rem;
  font-size: 0.7rem;
  color: var(--primary-highlight);
  font-family: "Questrial", sans-serif;
}

.host-name {
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  cursor: pointer;
  transition: color 0.2s ease;
}

.host-name:hover {
  color: var(--primary-highlight);
  text-decoration: underline;
}

.host-remove-btn {
  background: none;
  border: none;
  color: var(--secondary-highlight);
  cursor: pointer;
  padding: 0;
  display: flex;
  align-items: center;
  transition: color 0.2s ease;
  line-height: 1;
}

.host-remove-btn:hover {
  color: #ef4444;
}
</style>
