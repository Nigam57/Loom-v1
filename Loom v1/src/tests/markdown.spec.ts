import { describe, expect, it } from 'vitest';
import { renderMarkdown } from '@/shared/utils/markdown';

describe('renderMarkdown', () => {
  it('renders emphasis and inline code', () => {
    expect(renderMarkdown('The item is **"ship"** in `notes.txt`')).toBe(
      '<p>The item is <strong>&quot;ship&quot;</strong> in <code>notes.txt</code></p>\n'
    );
  });

  it('never passes raw HTML through', () => {
    expect(renderMarkdown('<img src=x onerror=alert(1)>')).not.toContain('<img');
  });

  it('highlights mentions in text but not in code', () => {
    const html = renderMarkdown('@Kai see `@notamention`');
    expect(html).toContain('<span class="md-mention">@Kai</span>');
    expect(html).toContain('<code>@notamention</code>');
  });

  it('wraps fenced code with a language label', () => {
    const html = renderMarkdown('```ts\nconst a = 1;\n```');
    expect(html).toContain('<span class="md-code-lang">ts</span>');
    expect(html).toContain('<pre><code>const a = 1;\n</code></pre>');
  });

  it('opens links outside the app', () => {
    expect(renderMarkdown('[docs](https://example.com)')).toContain('target="_blank"');
  });
});
