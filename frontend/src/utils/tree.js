export function buildTree(list) {
  const map = {},
    tree = [];
  list.forEach((n) => {
    map[n.id] = { ...n, children: [] };
  });
  list.forEach((n) => {
    if (n.parentId && map[n.parentId]) map[n.parentId].children.push(map[n.id]);
    else tree.push(map[n.id]);
  });
  const srt = (a, b) => (a.sortOrder || 0) - (b.sortOrder || 0);
  tree.sort(srt);
  Object.values(map).forEach((n) => n.children.sort(srt));
  return tree;
}
