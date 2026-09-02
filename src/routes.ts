const focusMarker = 'infra-test-evidence-route-focus';

function isInternalPageLink(anchor: HTMLAnchorElement): boolean {
  const target = new URL(anchor.href, window.location.href);
  return target.origin === window.location.origin
    && `${target.pathname}${target.search}` !== `${window.location.pathname}${window.location.search}`;
}

document.addEventListener('click', (event) => {
  const target = event.target;
  if (!(target instanceof Element)) return;
  const anchor = target.closest('a[href]');
  if (anchor instanceof HTMLAnchorElement && isInternalPageLink(anchor)) window.name = focusMarker;
});

function focusRouteHeading(): void {
  const heading = document.querySelector<HTMLElement>('main h1');
  const announcement = document.querySelector<HTMLElement>('#route-announcement');
  if (!heading) return;
  window.requestAnimationFrame(() => {
    heading.focus({ preventScroll: true });
    if (announcement) announcement.textContent = document.title;
  });
}

window.addEventListener('pageshow', (event) => {
  if (window.name === focusMarker || event.persisted) {
    focusRouteHeading();
  }
});
