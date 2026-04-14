// ==============================
// FLUXLOCK EPOCH ENGINE
// ==============================

export function runEpoch(nodes) {

  // Shuffle nodes (visual layer only)
  const shuffled = [...nodes].sort(() => Math.random() - 0.5);

  // Reassign visual IDs
  shuffled.forEach((node, index) => {
    node.displayId = index;
    node.id = index; // ⚠️ this changes every epoch
  });

  return shuffled;
}