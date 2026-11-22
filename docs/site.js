// Play showcase videos only while on screen (and not at all for reduced motion).
const videos = document.querySelectorAll("video");
if (matchMedia("(prefers-reduced-motion: reduce)").matches || !("IntersectionObserver" in window)) {
  videos.forEach((video) => (video.controls = true));
} else {
  const observer = new IntersectionObserver(
    (entries) => {
      for (const { target, isIntersecting } of entries) {
        if (isIntersecting) target.play().catch(() => (target.controls = true));
        else target.pause();
      }
    },
    { threshold: 0.25 },
  );
  videos.forEach((video) => observer.observe(video));
}

// Point the download buttons straight at the latest installer; the static
// releases/latest link stays as the fallback if the API is unavailable.
fetch("https://api.github.com/repos/lummasharp/Spectral-Viewer/releases/latest")
  .then((response) => (response.ok ? response.json() : Promise.reject(response.status)))
  .then((release) => {
    const installer = release.assets.find((asset) => asset.name.toLowerCase().endsWith(".exe"));
    if (!installer) return;
    const version = release.tag_name.replace(/^v/i, "");
    const size = (installer.size / 1024 / 1024).toFixed(1);
    document.querySelectorAll("[data-download]").forEach((link) => {
      link.href = installer.browser_download_url;
      link.textContent = `Download v${version} for Windows`;
    });
    const note = document.querySelector("[data-release-note]");
    const date = new Date(release.published_at).toLocaleDateString(undefined, {
      year: "numeric",
      month: "long",
      day: "numeric",
    });
    note.textContent = `${size} MB installer · released ${date}`;
    note.hidden = false;
  })
  .catch(() => {});
