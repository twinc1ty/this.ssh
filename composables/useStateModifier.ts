import type { State, Status } from "~/types/core-types";

export const useStateModifier = (
  state: State, 
  newStatus: Status, 
  newMessage?: string, 
  newData?: object,
  ) => {
    state.status = newStatus;
    if(newMessage) {
        state.message = newMessage;
    } 

    if(newData){
        state.data = newData;
    }
}
