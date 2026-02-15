import fs from "fs";
import path from "path";
import { cache } from "react";
import { compileMDX } from "next-mdx-remote/rsc";
import { mdxComponents } from "../../mdx-components";

const docsDir = path.join(process.cwd(), "..", "docs");

const getDoc = cache(async (slug: string) => {
  const filePath = path.join(docsDir, `${slug}.md`);
  const source = await fs.promises.readFile(filePath, "utf8");
  return compileMDX<{ title?: string }>({
    source,
    components: mdxComponents,
    options: { parseFrontmatter: true },
  });
});

export default async function DocsIndex() {
  const { content, frontmatter } = await getDoc("index");
  return (
    <article>
      {frontmatter?.title ? <h1>{frontmatter.title}</h1> : <h1>Documentation</h1>}
      {content}
    </article>
  );
}
