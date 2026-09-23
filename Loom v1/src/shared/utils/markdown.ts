// Chat markdown: agent replies are markdown; raw HTML stays disabled so agent output can never inject markup.
import MarkdownIt from 'markdown-it';

const MENTION_PATTERN = /(^|[\s(])(@[\p{L}\p{N}_.-]+)/gu;

const md = new MarkdownIt({ html: false, linkify: true, breaks: true, typographer: false });

const escapeHtml = md.utils.escapeHtml;

// Mentions only inside plain text runs, never inside code.
md.renderer.rules.text = (tokens, idx) =>
  escapeHtml(tokens[idx].content).replace(MENTION_PATTERN, '$1<span class="md-mention">$2</span>');

const defaultLinkOpen =
  md.renderer.rules.link_open ?? ((tokens, idx, options, _env, self) => self.renderToken(tokens, idx, options));

md.renderer.rules.link_open = (tokens, idx, options, env, self) => {
  tokens[idx].attrSet('target', '_blank');
  tokens[idx].attrSet('rel', 'noreferrer noopener');
  return defaultLinkOpen(tokens, idx, options, env, self);
};

md.renderer.rules.fence = (tokens, idx) => {
  const token = tokens[idx];
  const language = token.info.trim().split(/\s+/)[0] ?? '';
  const label = language ? `<span class="md-code-lang">${escapeHtml(language)}</span>` : '';
  return `<div class="md-code">${label}<pre><code>${escapeHtml(token.content)}</code></pre></div>`;
};

export const renderMarkdown = (source: string): string => md.render(source ?? '');
