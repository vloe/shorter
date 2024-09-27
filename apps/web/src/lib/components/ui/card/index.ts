import Root from "./card.svelte"
import Content from "./card-content.svelte"
import Description from "./card-description.svelte"
import Footer from "./card-footer.svelte"
import Header from "./card-header.svelte"
import Title from "./card-title.svelte"

export {
	Content,
	Content as CardContent,
	Description,
	Description as CardDescription,
	Footer,
	Footer as CardFooter,
	Header,
	Header as CardHeader,
	Root,
	//
	Root as Card,
	Title,
	Title as CardTitle,
}

export type HeadingLevel = "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
