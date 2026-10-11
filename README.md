# ChromaDisc

Welcome to ChromaDisc, a project I am pursuing with the following goals:

- Learn about the various CD-\* format specifications as defined in the [Rainbow Books](https://en.wikipedia.org/wiki/Rainbow_Books)
- Learn Rust. I already love rust as a language, but I know very little about low-level systems development with rust.
- Develop a replacement for `cdparanoia` that properly implements error correction, gap-detection, and HTOA (Hidden track one audio) support. Eventually, I would like ChromaDisc to be robust enough to serve as an open source replacement for Exact Audio Copy and X Lossless Decoder.

## AI Policy

Generally speaking, I think the Ghostty project takes a very [reasonable stance](https://github.com/ghostty-org/ghostty/blob/main/AI_POLICY.md) towards AI usage.

As for my personal stance, if you have met me you almost certainly know how I feel about AI, and if not, you can probably assume. With that said, when it comes to programming and this project in particular, I absolutely use AI-developement tools for a variety of tasks and I think it is worth explaining what I use it for and my approach to avoid slop-ifying my passion project.

**What I use it for**

1. Finding references: For some things it is a better Google search, and for this project it's helped me find a lot of old documentation.
2. Prose: I find it super useful for writing things like doc comments where the text is short enough to not overtly sound like slop.
3. Talking through design decisions and their consequences: I never let an agent make a design decision without first discussing it thoroughly. It does annoy me how much of a yes-man AI models can be, but I do find utility in bouncing ideas off of it to give me a better understanding of the consequences of my choices.
4. Bulk code writing: For well-established patterns where there are no design decisions to be made, I am comfortable letting an agent write the boilerplate for me to review.
5. Finding bugs: Arguably its best use-case, it is very good at reading many source files and finding bugs before testing does.

**My Approach**
In my experience, AI-development tools really want you to take the approach of throwing together a complete high-level plan and then have an agent do the rest of the coding, testing, etc. hands-off for you (because that's the approach that sells their narrative). I find that this almost always results in complete slop. If you genuinely put "don't come back until done" or "make no mistakes" at the end of your prompts, we probably won't get along.

I always use the latest programming-focused models, and despite this, the only circumstances under which an AI is able to achieve a result that I am satisfied with is with me rigorously guiding it through every detail. Oftentimes, I will arrive at my optimal solution myself before the AI does, just as a result of talking through the problem.

That being said, whenever an agent writes **ANYTHING**, whether it is prose, code, or otherwise, I read every line. I review, revise the code myself, and follow-up on every minor design decision that I find suboptimal. This is a slow, cyclical, incremental process, but it is what works for me and my standards of quality.

In cases where I have absolutely slop-hallucinated something, whether it be for a quick prototype or otherwise, I make it obvious and explicit, with full intentions of going back to replace it with something better.

## libcdio

After a not-so-thorough review, I have determined that I will not utilize the rust bindings for `libcdio` in this project. In short, after an initial investigation, I found a few confusing/questionable design choices (libcdio's own GNU manual documents admitted as such in a few places) and decided writing my own library would be better both for development and my own learning, rather than trying to shoe-horn in an existing C library. However, it will be a great reference and point of comparison.
