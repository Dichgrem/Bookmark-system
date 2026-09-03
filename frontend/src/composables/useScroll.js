import { ref } from "vue";

export function useScroll(contentRef, selectedCategoryId) {
  const showBackToTop = ref(false);

  const onContentScroll = () => {
    showBackToTop.value = (contentRef.value?.scrollTop || 0) > 300;
  };

  const scrollToTop = () => {
    contentRef.value?.scrollTo({ top: 0, behavior: "smooth" });
  };

  const scrollToGroup = (cateId) => {
    selectedCategoryId.value = cateId;
    const el = document.getElementById(`group-${cateId}`);
    const container = contentRef.value;
    if (el && container) {
      const cr = container.getBoundingClientRect();
      const er = el.getBoundingClientRect();
      container.scrollTo({
        top: container.scrollTop + er.top - cr.top - 20,
        behavior: "smooth",
      });
    }
  };

  return { showBackToTop, onContentScroll, scrollToTop, scrollToGroup };
}
