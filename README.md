# Rewind-Card

![https://github.com/Stvchm9703/rewind-card/raw/main/doc/img/rewind-cover-2.png](https://github.com/Stvchm9703/rewind-card/raw/main/doc/img/rewind-cover-2.png)

### ReWind-Card

inspiration by UNOCSS, so this project is like a UNO reverse card, converts to CSS property to Tailwind CSS class, and 

- Table of Contents

## About The Project

In my experience in frontend development, functional CSS frameworks (like Tailwind CSS, WindiCSS, or UNOCSS) have a nice performance in Page Speed and Development Experience (DX). However, it won’t be easy to rewrite a new page when the project is ongoing and launched when the learning cost is still high. So, I make a transformer program (currently are CLI program that 😅 ) to convert the CSS property to Tailwind-like CSS class (or, say, tailwind-token)

 So in my case, I have a Vue Component Library with many SCSS files, and one day I felt when the libs was implemented for the Nuxt project, those projects were not working as light-weight as expected! Neither the flexibility of applying the design token nor the overall performance caused by the CSS file. And I start thinking about rewriting the style files and reconsidering the page distribution in the project-based projects. 

([back to top](https://github.com/Stvchm9703/rewind-card#readme-top))

## Usage

1. clone this project 
2. create a directory `input-src/` under the project, and put the CSS file under the input-src directory
3. create a directory `out-tw-token/` under the project for the result JSON file.
4. execute the main script, `cargo run`
5. Ta Da~, the token is exported as array form. 

([back to top](https://github.com/Stvchm9703/rewind-card#readme-top))

## Roadmap

- [x]  create a raw concept program
- [ ]  fixing the media query logic handle
- [ ]  fixing the class / pseudo-element replacing the logic
- [ ]  feature: add custom `tailwind.config.js` rule/variable
- [ ]  feature: export new config file for unsolve token
- [ ]  convert HTML / Vue / Svelte
    - [ ]  convert style to token
    - [ ]  convert the template part code’s class name to the token,

See the [open issues](https://github.com/Stvchm9703/rewind-card/issue) for a full list of proposed features (and known issues).

([back to top](https://github.com/Stvchm9703/rewind-card#readme-top))

## Contributing

Contributions make the open-source community an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement". Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

([back to top](https://github.com/Stvchm9703/rewind-card#readme-top))

## License

So call MIT License, just better for contributing.

([back to top](https://github.com/Stvchm9703/rewind-card#readme-top))

## Contact

Steven Chm - [stv.chm@gmail.com](mailto:stv.chm@gmail.com)

Project Link: [https://github.com/Stvchm9703/rewind-card](https://github.com/your_username/repo_name)

([back to top](https://github.com/Stvchm9703/rewind-card#readme-top))

## Acknowledgments

Use this list of resources you may be interested in: 

[Tailwind CSS](https://tailwindcss.com/):  the origin of everything 

[Windi CSS](https://windicss.org/): the better engine in the conception of JIT.

[UNOCSS](https://github.com/unocss/unocss): the greater one JIT  of Atomic CSS Engine. Most of the code conversion is based on this engine, and makes sure the converted token can execute 

[lightningcss](https://docs.rs/lightningcss/1.0.0-alpha.35/lightningcss/) : the main CSS parser for this project

[tailwind-rs](https://github.com/oovm/tailwind-rs) : maybe better community for a similar project 

([back to top](https://github.com/Stvchm9703/rewind-card#readme-top))