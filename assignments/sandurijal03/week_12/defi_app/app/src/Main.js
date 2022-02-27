import styled from 'styled-components';
import { HeroSection } from './components/HeroSection';

const Wrapper = styled.div`
  height: 100vh;
  background-color: #10121a;
  color: white;
  display: flex;
  justify-content: center;
  align-items: center;
`;

export const Main = () => {
  return (
    <Wrapper>
      <HeroSection />
    </Wrapper>
  );
};
