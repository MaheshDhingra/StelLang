import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

const FeatureList = [
  {
    title: 'Simplicity',
    Svg: require('@site/static/img/stel.svg').default,
    description: (
      <>
        stel is designed with simplicity in mind. It's easy to learn and understand, making it perfect for beginners. Yet, it's powerful enough to handle complex tasks, making it a great choice for experienced developers as well.
      </>
    ),
  },
  {
    title: 'Efficiency',
    Svg: require('@site/static/img/stel.svg').default,
    description: (
      <>
        stel is built for speed. It's optimized for performance, ensuring that your programs run as fast as possible. With stel, you can focus on writing great code, knowing that it will be executed efficiently.
      </>
    ),
  },
  {
    title: 'Versatility',
    Svg: require('@site/static/img/stel.svg').default,
    description: (
      <>
        <span> stel </span> is a versatile language. It supports multiple programming paradigms, allowing you to choose the best approach for each task. Whether you prefer procedural, object-oriented, or functional programming, stel has you covered.
      </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
        <img src={styles.featureSvg}  alt="" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
