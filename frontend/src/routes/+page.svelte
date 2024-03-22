<script>
    import { Socials } from '@/config';
    import Icon from '@/lib/Icon.svelte';
</script>

<h2>About me</h2>

<p>
    My name is Sebastian Nielsen, and I am currently pursuing a Master's degree
    at Chalmers University of Technology with a focus on Algorithms, Languages,
    and Logic.
</p>
<p>
    Right now, I'm in writing my master thesis about concurrent priority queues
    in Rust. Focusing both on the performance and development aspects of Rust,
    while investigating improvements to the algorithms.
</p>

<h2>Personal Projects</h2>
<ul>
    <li>
        I maintain a small <a href="https://www.proxmox.com/en/">Proxmox</a>
        server to dabble in self-Hosting. Beyond this website, I also host
        <a href="https://about.gitea.com/">Gitea</a>,
        <a href="https://about.gitlab.com/solutions/continuous-integration/"
            >GitLab CI</a
        >, and
        <a href="https://www.home-assistant.io/">Home Assistant</a>, among
        others.
    </li>
    <li>
        <a href="https://github.com/sebostien/raytracer_rs">raytracer_rs</a> Simple
        ray tracer with material and reflection support.
    </li>
    <li>
        <a href="https://github.com/sebostien/automata_rust/">automata_rust</a> Implementations
        and visualizations of some finite-state machines. Just an excuse to work
        on some graph theory.
    </li>
</ul>

<h2>Languages and Technologies</h2>
<p>
    I am passionate about software development and enjoy exploring all aspects
    of it. While my thesis is heavily focused on Rust, I try to broaden my scope
    when time permits.
</p>
<ul>
    <li>
        <b>Rust:</b> The preferred language. Performance, types, and safety. I write
        most projects in Rust.
    </li>
    <li>
        <b>C:</b> I have developed various projects in C, mainly in systems programming
        applications.
    </li>
    <li>
        <b>TypeScript:</b> Various projects, both as a backend with Node.js and express.js,
        but also through Svelte or React.
    </li>
    <li><b>Java:</b> Mostly used during studies.</li>
    <li><b>Docker:</b> Self-Hosting all things the docker way!</li>
    <li><b>Other:</b> Python, C#, Lua, Haskell, Agda, SQL, Bash</li>
</ul>

<h2>Development Setup</h2>
<p>
    <a href="https://neovim.io/">NeoVim</a> and
    <a href="https://www.zsh.org/">zsh</a>
    inside
    <a href="https://github.com/tmux/tmux">tmux</a>
    tiled with <a href="https://xmonad.org/">XMonad</a> on
    <a href="https://fedoraproject.org/">Fedora</a>
</p>

<h2>You can reach me at:</h2>
<ul>
    {#each Socials as social}
        <li>
            <div class="inline">
                <Icon
                    classes="inline-block w-6 translate-y-1"
                    svg={social.icon}
                    alt={social.alt}
                />
                {social.name}:
            </div>
            <a href={social.link} class="">{social.username}</a>
        </li>
    {/each}
</ul>

<style>
    h2 {
        font-size: 2.4rem;
        font-weight: 700;
    }

    p {
        font-size: 1.2rem;
        padding-top: 0.5rem;
        padding-bottom: 0.5rem;
    }

    ul {
        font-size: 1.2rem;
        padding-left: 1rem;
        padding-bottom: 1rem;
        list-style-type: disc;
    }

    li {
        padding-top: 0.2rem;
        padding-bottom: 0.2rem;
    }

    a {
        color: #13ce66;
    }
</style>
