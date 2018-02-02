#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define DEBUG_MODE 1

#define SET_OPCODE 0
#define ADD_OPCODE 1
#define MUL_OPCODE 2
#define MOD_OPCODE 4
#define SND_OPCODE 5
#define RCV_OPCODE 6
#define JGZ_OPCODE 7

typedef struct {
  int is_immediate;
  char reg;
  int immediate;
} instruction_arg;

typedef struct {
  int opcode;
  instruction_arg first_arg;
  instruction_arg second_arg;
} instruction;

typedef struct {
  int num_instructions;
  instruction instructions[100];
} program;


#define OUTPUT_BUFFER_CAPACITY 1000
typedef struct {
  program code;
  long registers[26];
  int ip; // instruction pointer
  int pid;

  // output is a circular buffer to hold pending msgs
  long output[OUTPUT_BUFFER_CAPACITY];
  int output_count;
  int output_index;
  int send_count;

  int blocking;
  char receive_register;

  int terminated;
} process;

long get_register(process *proc, char reg) {
  return proc->registers[reg-'a'];
}

void set_register(process *proc, char reg, long value) {
  proc->registers[reg-'a'] = value;
}

void init_registers(process *proc) {
  for(char i='a'; i<='z'; i++) {
    set_register(proc, i, 0);
  }
}

int has_pending_output(process *p) {
  return p->output_count > 0;
}

void enqueue_output(process *proc, long output) {
  if (proc->output_count > OUTPUT_BUFFER_CAPACITY) {
    printf("Output buffer capacity exceeded\n");
    exit(1);
  }
  int new_index = (proc->output_index + proc->output_count) % OUTPUT_BUFFER_CAPACITY;
  proc->output[new_index] = output;
  proc->output_count += 1;
}

void debug_output(process *proc) {
  int initial_pass = 1;
  printf("output: ");
  for(int i=proc->output_index; i<proc->output_index + proc->output_count; i++) {
    int index = i % OUTPUT_BUFFER_CAPACITY;
    if(initial_pass) {
      printf("%ld", proc->output[index]);
      initial_pass = 0;
    } else {
      printf(",%ld", proc->output[index]);
    }
  }
}

void dump_process(process *proc) {
  printf("pid: %d\t", proc->pid);
  for(char i='a'; i<='z'; i++) {
    printf("%c: %ld\t", i, get_register(proc, i));
  }
  printf("ip: %d\tblocking: %d\trcv_reg: %c\tterminated: %d\t", proc->ip, proc->blocking, proc->receive_register, proc->terminated);
  debug_output(proc);
  printf("\tsend_count: %d", proc->send_count);

  printf("\n");
}

void debug_process(process *proc) {
  if(DEBUG_MODE) {
    dump_process(proc);
  }
}

void read_instruction_arg(instruction_arg *arg) {
  if (fscanf(stdin, " %d", &(arg->immediate))) {
    arg->is_immediate = 1;
  } else {
    arg->is_immediate = 0;
    fscanf(stdin, " %c", &(arg->reg));
  }
}

instruction read_instruction() {
  instruction i = {};
  char opcode[4] = {0};
  fscanf(stdin, "%3s", opcode);
  if(strcmp("snd", opcode) == 0) {
    i.opcode = SND_OPCODE;
    read_instruction_arg(&(i.first_arg));
  } else if (strcmp("set", opcode) == 0) {
    i.opcode = SET_OPCODE;
    read_instruction_arg(&(i.first_arg));
    read_instruction_arg(&(i.second_arg));
  } else if (strcmp("add", opcode) == 0) {
    i.opcode = ADD_OPCODE;
    read_instruction_arg(&(i.first_arg));
    read_instruction_arg(&(i.second_arg));
  } else if (strcmp("mul", opcode) == 0) {
    i.opcode = MUL_OPCODE;
    read_instruction_arg(&(i.first_arg));
    read_instruction_arg(&(i.second_arg));
  } else if (strcmp("mod", opcode) == 0) {
    i.opcode = MOD_OPCODE;
    read_instruction_arg(&(i.first_arg));
    read_instruction_arg(&(i.second_arg));
  } else if (strcmp("rcv", opcode) == 0) {
    i.opcode = RCV_OPCODE;
    read_instruction_arg(&(i.first_arg));
  } else if (strcmp("jgz", opcode) == 0) {
    i.opcode = JGZ_OPCODE;
    read_instruction_arg(&(i.first_arg));
    read_instruction_arg(&(i.second_arg));
  }

  return i;
}

void print_instruction_arg(instruction_arg arg) {
  if(arg.is_immediate) {
    fprintf(stdout, "%d", arg.immediate);
  } else {
    fprintf(stdout, "%c", arg.reg);
  }
}

void debug_instruction(instruction i) {
  if(!DEBUG_MODE) {
    return;
  }
  switch(i.opcode) {
    case SET_OPCODE:
      fprintf(stdout, "set %c ", i.first_arg.reg);
      print_instruction_arg(i.second_arg);
      fprintf(stdout, "\n");
      break;
    case ADD_OPCODE:
      fprintf(stdout, "add %c ", i.first_arg.reg);
      print_instruction_arg(i.second_arg);
      fprintf(stdout, "\n");
      break;
    case MUL_OPCODE:
      fprintf(stdout, "mul %c ", i.first_arg.reg);
      print_instruction_arg(i.second_arg);
      fprintf(stdout, "\n");
      break;
    case MOD_OPCODE:
      fprintf(stdout, "mod %c ", i.first_arg.reg);
      print_instruction_arg(i.second_arg);
      fprintf(stdout, "\n");
      break;
    case SND_OPCODE:
      fprintf(stdout, "snd ");
      print_instruction_arg(i.first_arg);
      fprintf(stdout, "\n");
      break;
    case RCV_OPCODE:
      fprintf(stdout, "rcv ");
      print_instruction_arg(i.first_arg);
      fprintf(stdout, "\n");
      break;
    case JGZ_OPCODE:
      fprintf(stdout, "jgz ");
      print_instruction_arg(i.first_arg);
      fprintf(stdout, " ");
      print_instruction_arg(i.second_arg);
      fprintf(stdout, "\n");
      break;
  }
}

program load_code() {
  program prog;
  int num_instructions = 0;
  instruction *i = malloc(sizeof(instruction));
  *i = read_instruction();
  while (!feof(stdin)) {
    prog.instructions[prog.num_instructions++] = *i;
    debug_instruction(*i);
    *i = read_instruction();
  }
  return prog;
}

void set_instr(process *p, instruction *i){
  if (i->second_arg.is_immediate) {
    set_register(p, i->first_arg.reg, i->second_arg.immediate);
  } else {
    set_register(p, i->first_arg.reg, get_register(p, i->second_arg.reg));
  }
  p->ip++;
}

void add_instr(process *p, instruction *i){
  if (i->second_arg.is_immediate) {
    set_register(p, i->first_arg.reg, get_register(p, i->first_arg.reg) + i->second_arg.immediate);
  } else {
    set_register(p, i->first_arg.reg, get_register(p, i->first_arg.reg) + get_register(p, i->second_arg.reg));
  }
  p->ip++;
}

void mul_instr(process *p, instruction *i){
  if (i->second_arg.is_immediate) {
    set_register(p, i->first_arg.reg, get_register(p, i->first_arg.reg) * i->second_arg.immediate);
  } else {
    set_register(p, i->first_arg.reg, get_register(p, i->first_arg.reg) * get_register(p, i->second_arg.reg));
  }
  p->ip++;
}

void mod_instr(process *p, instruction *i){
  if (i->second_arg.is_immediate) {
    set_register(p, i->first_arg.reg, get_register(p, i->first_arg.reg) % i->second_arg.immediate);
  } else {
    set_register(p, i->first_arg.reg, get_register(p, i->first_arg.reg) % get_register(p, i->second_arg.reg));
  }
  p->ip++;
}

void snd_instr(process *p, instruction *i){
  if (i->first_arg.is_immediate) {
    enqueue_output(p, i->first_arg.immediate);
  } else {
    enqueue_output(p, get_register(p, i->first_arg.reg));
  }
  p->send_count++;
  p->ip++;
}

void rcv_instr(process *p, instruction *i){
  p->blocking = 1;
  p->receive_register = i->first_arg.reg;
  p->ip++;
}

void jgz_instr(process *p, instruction *i){
  if(i->first_arg.is_immediate) {
    if(i->first_arg.immediate <= 0) {
      p->ip++;
      return;
    }
  } else {
    if(get_register(p, i->first_arg.reg) <= 0) {
      p->ip++;
      return;
    }
  }

  int delta = 0;
  if(i->second_arg.is_immediate) { 
    delta = i->second_arg.immediate;
  } else {
    delta = get_register(p, i->second_arg.reg);
  }

  p->ip += delta;
}

void receive_input(process *receiving_process, long value) {
  set_register(receiving_process, receiving_process->receive_register, value);
  receiving_process->receive_register = 0;
  receiving_process->blocking = 0;
}

long shift_output(process *p) {
  long value = p->output[p->output_index];
  p->output_index = (p->output_index + 1) % OUTPUT_BUFFER_CAPACITY;
  p->output_count--;
  return value;
}

void clear_output(process *p) {
  p->output_index = 0;
  p->output_count = 0;
}

void interprocess_msg(process *receiving_process, process *sending_process) {
  receive_input(receiving_process, shift_output(sending_process));
}

void init_process(process *p) {
  init_registers(p);
  p->output_index = OUTPUT_BUFFER_CAPACITY -2;
  p->output_count = 0;
  p->send_count = 0;

  p->receive_register = 0;
  p->blocking = 0;
  p->ip = 0;
  p->terminated = 0;
}

void step(process *p) {
  instruction *i = &(p->code.instructions[p->ip]);
  debug_instruction(*i);
  switch(i->opcode) {
    case SET_OPCODE:
      set_instr(p, i);
      break;
    case ADD_OPCODE:
      add_instr(p, i);
      break;
    case MUL_OPCODE:
      mul_instr(p, i);
      break;
    case MOD_OPCODE:
      mod_instr(p, i);
      break;
    case SND_OPCODE:
      snd_instr(p, i);
      break;
    case RCV_OPCODE:
      rcv_instr(p, i);
      break;
    case JGZ_OPCODE:
      jgz_instr(p, i);
      break;
  }
  if(p->ip <0 || p->ip >= p->code.num_instructions) {
    p->terminated = 1;
  }
}

void deadlock(process *p1, process *p2) {
  printf("Processes deadlocked.\nCore dump:\n");
  dump_process(p1);
  dump_process(p2);
  exit(0);
}

void shutdown(process *p1, process *p2) {
  printf("Both processes exited.\nFinal state:\n");
  debug_process(p1);
  debug_process(p2);
  exit(0);
}

void execute_duet(program prog) {
  process p1;
  process p2;
  p1.code = prog;
  p2.code = prog;
  init_process(&p1);
  init_process(&p2);
  p1.pid = 0;
  set_register(&p1, 'p', 0);
  p2.pid = 1;
  set_register(&p2, 'p', 1);

  for(;;) {
    if (p1.blocking && p2.blocking && !has_pending_output(&p1) && !has_pending_output(&p2)) {
      deadlock(&p1, &p2);
    }
    if(p1.terminated && p2.terminated) {
      shutdown(&p1, &p2);
    }

    if(!p1.terminated) {
      if (!p1.blocking) {
        if(DEBUG_MODE) {
          printf("P1 not blocking; stepping P1\n");
        }
        step(&p1);
        debug_process(&p1);
      } else if (has_pending_output(&p2)) {
        if(DEBUG_MODE) {
          printf("P1 blocking; P2 has output; sending output to P1\n");
        }
        interprocess_msg(&p1, &p2);
        debug_process(&p1);
        debug_process(&p2);
      }
    }
    if(!p2.terminated) {
      if (!p2.blocking) {
        if(DEBUG_MODE) {
          printf("p2 not blocking; stepping p2\n");
        }
        step(&p2);
        debug_process(&p2);
      } else if (has_pending_output(&p1)) {
        if(DEBUG_MODE) {
          printf("p2 blocking; p1 has output; sending output to P2\n");
        }
        interprocess_msg(&p2, &p1);
        debug_process(&p1);
        debug_process(&p2);
      }
    }

  }
}

int main(int num_args, char** args) {
  program prog = load_code();
  execute_duet(prog);
}
