import { GetStaticProps } from "next";

export const getStaticProps: GetStaticProps<StartPageProps> = async () => {
	const resp = await fetch("http://backend:1337");
	const { text } = await resp.json();

	return {
		props: {
			text,
		},
	};
};

interface StartPageProps {
	text: string;
}

const StartPage = ({ text }: StartPageProps) => {
	return <h1>{text}</h1>;
};

export default StartPage;
