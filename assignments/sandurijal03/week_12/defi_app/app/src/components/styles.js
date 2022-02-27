import styled from 'styled-components';

const Wrapper = styled.main``;

const Screen = styled.div`
  background-color: white;
  width: 350px;
  height: 60px;
  color: black;
  text-align: center;
  font-size: 2rem;
  font-weight: bold;
  margin-right: 100px;
`;

const Button = styled.button`
  padding: 15px;
  background-color: blue;
  font-size: 1rem;
  color: white;
  border: none;
  border-radius: 10px;
  margin-left: 10px;
  cursor: pointer;
`;

const Top = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
`;

const Title = styled.h1`
  font-size: 4rem;
  letter-spacing: 10px;
`;

const Description = styled.p`
  margin-bottom: 10px;
  font-size: 2rem;
`;

const Bottom = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border: 2px solid;
  margin-top: 20px;
`;

const InputContainer = styled.div`
  margin: 20px 0;
`;

const Input = styled.input`
  padding: 10px;
  outline: none;
  font-size: 1.5rem;
  background-color: transparent;
  border-radius: 10px;
  color: white;
`;

const DepositButton = styled(Button)`
  background-color: green;
`;

const BorrowButton = styled(Button)`
  background-color: orange;
`;

export {
  BorrowButton,
  DepositButton,
  Input,
  InputContainer,
  Bottom,
  Title,
  Description,
  Top,
  Wrapper,
  Screen,
  Button,
};
