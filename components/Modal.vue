<template>
  <div v-if="isOpen" class="modal-overlay" @click="handleBackdropClick">
    <div class="modal-wrapper" @click.stop>
      <!-- Modal Header -->
      <div class="modal-header">
        <slot name="header">
          <h2 class="modal-title">Modal Title</h2>
        </slot>
        <button
          @click="closeModal"
          class="modal-close-btn"
          aria-label="Close modal"
        >
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
            />
          </svg>
        </button>
      </div>

      <div class="modal-content">
        <slot name="content">
          <p>MOdal</p>
        </slot>
      </div>

      <div v-if="$slots.footer" class="modal-footer">
        <slot name="footer" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  isOpen: boolean;
  closeOnBackdrop?: boolean;
}

interface Emits {
  (e: "close"): void;
}

const props = withDefaults(defineProps<Props>(), {
  closeOnBackdrop: true,
});

const emit = defineEmits<Emits>();

const closeModal = () => {
  emit("close");
};

const handleBackdropClick = () => {
  if (props.closeOnBackdrop) {
    closeModal();
  }
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.75);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 50;
  padding: 1rem;
}

.modal-wrapper {
  background-color: var(--primary-bg);
  border: 2px solid var(--primary-highlight);
  border-radius: 0.75rem;
  width: 100%;
  max-width: 32rem;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem 1.5rem 0 1.5rem;
  border-bottom: 1px solid var(--secondary-highlight);
  padding-bottom: 1rem;
}

.modal-title {
  color: var(--primary-highlight);
  font-size: 1.25rem;
  font-weight: 600;
  margin: 0;
}

.modal-close-btn {
  background: none;
  border: none;
  color: var(--secondary-highlight);
  cursor: pointer;
  padding: 0.25rem;
  border-radius: 0.25rem;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal-close-btn:hover {
  color: var(--primary-highlight);
  background-color: var(--secondary-bg);
}

.modal-content {
  padding: 1.5rem;
  flex: 1;
}

.modal-footer {
  padding: 0 1.5rem 1.5rem 1.5rem;
  border-top: 1px solid var(--secondary-highlight);
  padding-top: 1rem;
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
}

/* Responsive adjustments */
@media (max-width: 640px) {
  .modal-wrapper {
    max-width: 100%;
    margin: 0.5rem;
  }

  .modal-header,
  .modal-content,
  .modal-footer {
    padding-left: 1rem;
    padding-right: 1rem;
  }
}
</style>
